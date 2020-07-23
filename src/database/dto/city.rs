use crate::database::dto::Dto;
use crate::database::infra::repository::Repository;
use diesel::prelude::*;
use diesel::{Connection, MysqlConnection, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Deserialize)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub postal_code: i32,
    pub country_id: i32,
}

pub struct CitiesRepository<'a, C: Connection> {
    connection: &'a C,
}

impl Dto for City {}

impl City {
    pub fn new(id: i32, name: String, postal_code: i32, country_id: i32) -> Self {
        City {
            id,
            name,
            postal_code,
            country_id,
        }
    }
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
            .unwrap_or_else(|_| panic!("Failed to retrieve country {}", idp))
    }
}

