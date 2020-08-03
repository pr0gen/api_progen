use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Connection, Insertable, MysqlConnection, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::dto;
use crate::database::dto::city::City;
use crate::database::dto::Dto;
use crate::database::infra::repository::Repository;
use crate::database::schema::place;

pub fn as_place(longitude: f32, latitude: f32, city_id: i32, nb_place: i32) -> Place {
    Place {
        id: 0,
        longitude,
        latitude,
        city_id,
        nb_place,
        created_at: dto::now(),
        updated_at: dto::now(),
    }
}

#[test]
pub fn should_convert() {
    let expected = Place::new(0, 1.0, 2.0, 3, 4, dto::now(), dto::now());
    let actual = as_place(1.0, 2.0, 3, 4);
    assert_eq!(actual.id, expected.id);
    assert_eq!(actual.longitude, expected.longitude);
    assert_eq!(actual.latitude, expected.latitude);
    assert_eq!(actual.city_id, expected.city_id);
    assert_eq!(actual.nb_place, expected.nb_place);
}

#[derive(Serialize, Queryable, Deserialize)]
pub struct Place {
    id: i32,
    longitude: f32,
    latitude: f32,
    city_id: i32,
    nb_place: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "place"]
struct InsertablePlace {
    longitude: f32,
    latitude: f32,
    city_id: i32,
    nb_place: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

pub struct PlacesRepository<'a, C: Connection> {
    connection: &'a C,
}

impl Dto for Place {}

impl Place {
    pub fn new(
        id: i32,
        longitude: f32,
        latitude: f32,
        city_id: i32,
        nb_place: i32,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Place {
            id,
            longitude,
            latitude,
            city_id,
            nb_place,
            created_at,
            updated_at,
        }
    }

    pub fn get_longitude(&self) -> f32 {
        self.longitude
    }

    pub fn get_latitude(&self) -> f32 {
        self.latitude
    }

    pub fn get_nb_place(&self) -> i32 {
        self.nb_place
    }
}

impl InsertablePlace {
    fn from_place(place: &Place) -> InsertablePlace {
        InsertablePlace {
            longitude: place.longitude,
            latitude: place.latitude,
            city_id: place.city_id,
            nb_place: place.nb_place,
            created_at: place.created_at,
            updated_at: place.updated_at,
        }
    }
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
