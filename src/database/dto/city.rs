use serde::{Serialize, Deserialize};
use diesel::{Queryable, Connection, MysqlConnection};
use crate::database::dto::Dto;
use crate::database::infra::entities_handlers::EntityHandler;
use diesel::prelude::*;

#[derive(Serialize, Queryable, Deserialize)]
pub struct City {
    pub id: i32,
    pub name: String,
    pub postal_code : i32,
    pub country_id: i32,
}

pub struct CitiesHandler<C : Connection> {
    connection : C
}

impl Dto for City {}

impl City {
    pub fn new(id :i32, name: String, postal_code: i32, country_id : i32) -> Self {
        City {
            id,
            name,
            postal_code,
            country_id
        }
    }
}

impl EntityHandler<MysqlConnection, City> for CitiesHandler<MysqlConnection> {
    fn new(connection: MysqlConnection) -> Self {
        CitiesHandler {connection}
    }

    fn select(&self) -> Vec<City> {
        use super::super::schema::city::dsl::*;
        city.
            load::<City>(&self.connection)
            .expect("Failed to retrieve all data")
    }

    fn select_by_id(&self, idp: i32) -> Vec<City> {
        use super::super::schema::city::dsl::*;
        city.
            filter(id.eq(idp))
            .load::<City>(&self.connection)
            .expect(format!("Failed to retrieve country {}", idp).as_str())
    }
}
