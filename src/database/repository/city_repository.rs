use diesel::prelude::*;
use diesel::{Connection, MysqlConnection};

use crate::database::dto::city::{City, InsertableCity};
use crate::database::infra::repository::Repository;
use crate::database::schema::city;

pub struct CitiesRepository<'a, C: Connection> {
    connection: &'a C,
}

impl<'a> Repository<'a, MysqlConnection, City> for CitiesRepository<'a, MysqlConnection> {
    fn new(connection: &'a MysqlConnection) -> Self {
        CitiesRepository { connection }
    }

    fn select(&self) -> Vec<City> {
        use super::super::schema::city::dsl::*;
        city.load::<City>(self.connection)
            .expect("Failed to retrieve all data")
    }

    fn select_by_id(&self, idp: i32) -> Vec<City> {
        use super::super::schema::city::dsl::*;
        city.filter(id.eq(idp))
            .load::<City>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to retrieve city {}", idp))
    }

    fn insert(&self, data: &City) -> QueryResult<usize> {
        diesel::insert_into(city::table)
            .values(&InsertableCity::from_city(data))
            .execute(self.connection)
    }

    fn insert_multiples(&self, data: &[City]) -> QueryResult<usize> {
        let insert_cities: Vec<InsertableCity> = data
            .iter()
            .map(|city| InsertableCity::from_city(city))
            .collect();
        diesel::insert_into(city::table)
            .values(insert_cities)
            .execute(self.connection)
    }
}

impl<'a> CitiesRepository<'a, MysqlConnection> {
    pub fn select_by_name(&self, city_name: &str) -> Vec<City> {
        use crate::database::schema::city::dsl::*;
        city.filter(name.eq(city_name))
            .load::<City>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to find city {} in database", city_name))
    }
}
