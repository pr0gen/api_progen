use crate::database::dto::Dto;
use crate::database::infra::repository::Repository;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel::{Connection, MysqlConnection, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Deserialize)]
pub struct Place {
    pub id: i32,
    pub longitude: f32,
    pub latitude: f32,
    pub city_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Place {
            id,
            longitude,
            latitude,
            city_id,
            created_at,
            updated_at,
        }
    }
}

impl<'a> Repository<'a, MysqlConnection, Place> for PlacesRepository<'a, MysqlConnection> {
    fn new(connection: &'a MysqlConnection) -> Self {
        PlacesRepository { connection }
    }

    fn select(&self) -> Vec<Place> {
        use super::super::schema::place::dsl::*;
        place
            .load::<Place>(self.connection)
            .expect("Failed to retrieve all data")
    }

    fn select_by_id(&self, idp: i32) -> Vec<Place> {
        use super::super::schema::place::dsl::*;
        place
            .filter(id.eq(idp))
            .load::<Place>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to retrieve place {}", idp))
    }
}

