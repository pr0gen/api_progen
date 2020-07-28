use chrono::NaiveDateTime; 
use diesel::prelude::*; 
use diesel::{Connection, MysqlConnection, Queryable, Insertable};
use serde::{Deserialize, Serialize};

use crate::database::schema::place;
use crate::database::dto::Dto;
use crate::database::infra::repository::Repository;

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
#[table_name ="place"]
pub struct InsertablePlace {
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
}

impl InsertablePlace {

    fn from_place(place: Place) -> InsertablePlace {
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

    fn insert(&self, data: Place) -> QueryResult<usize>{
        let data: Place = data;
        diesel::insert_into(place::table)
            .values(&InsertablePlace::from_place(data))
            .execute(self.connection)
    }
}

