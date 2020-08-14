use diesel::prelude::*;
use diesel::{Connection, MysqlConnection};

use crate::database::dto::city::City;
use crate::database::dto::place::{InsertablePlace, Place};
use crate::database::infra::repository::Repository;
use crate::database::schema::place;

pub struct PlacesRepository<'a, C: Connection> {
    connection: &'a C,
}

impl<'a> Repository<'a, MysqlConnection, Place> for PlacesRepository<'a, MysqlConnection> {
    fn new(connection: &'a MysqlConnection) -> Self {
        PlacesRepository { connection }
    }

    fn select(&self) -> Vec<Place> {
        use crate::database::schema::place::dsl::*;
        place
            .load::<Place>(self.connection)
            .expect("Failed to retrieve all data")
    }

    fn select_by_id(&self, idp: i32) -> Vec<Place> {
        use crate::database::schema::place::dsl::*;
        place
            .filter(id.eq(idp))
            .load::<Place>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to retrieve place {}", idp))
    }

    fn insert(&self, data: &Place) -> QueryResult<usize> {
        diesel::insert_into(place::table)
            .values(&InsertablePlace::from_place(data))
            .execute(self.connection)
    }

    fn insert_multiples(&self, data: &[Place]) -> QueryResult<usize> {
        let insert_place: Vec<InsertablePlace> = data
            .iter()
            .map(|place| InsertablePlace::from_place(place))
            .collect();
        diesel::insert_into(place::table)
            .values(insert_place)
            .execute(self.connection)
    }
}

impl<'a> PlacesRepository<'a, MysqlConnection> {
    pub fn select_by_city(&self, city: &City) -> Vec<Place> {
        use crate::database::schema::place::dsl::*;
        place
            .filter(city_id.eq(city.get_id()))
            .load::<Place>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to find places for city {}", city.get_name()))
    }
}

#[test]
fn should_insert_and_select() {
    use crate::database::dto::place::as_place;
    use crate::database::infra::db_pool;
    use crate::router;
    use diesel::result::Error;
    let to_insert = as_place(1.0, 2.0, 1, 10);
    let connection = db_pool::create_connexion(router::test_data_base_url().as_str());
    connection.test_transaction::<_, Error, _>(|| {
        let repository = PlacesRepository::new(&connection);
        repository.insert(&to_insert)?;

        use crate::database::schema::place::table;
        let all = table.select(place::city_id).load::<i32>(&connection)?;

        assert!(all.contains(&1));
        Ok(())
    });
}
