use serde::{Serialize, Deserialize};
use crate::database::dto::Dto;
use diesel::{Queryable, Connection, MysqlConnection};
use crate::database::infra::entities_handlers::EntityHandler;
use diesel::prelude::*;


#[derive(Serialize, Queryable, Deserialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
}

pub struct CountriesHandler<C: Connection> {
    connection: C
}

impl Dto for Country {}

impl Country {
    pub fn new(id: i32, name: String) -> Self {
        Country {id, name}
    }
}

impl EntityHandler<MysqlConnection, Country> for CountriesHandler<MysqlConnection> {
    fn new(connection: MysqlConnection) -> Self {
        CountriesHandler {connection}
    }

    fn select(&self) -> Vec<Country> {
        use super::super::schema::country::dsl::*;
        country.
            load::<Country>(&self.connection)
            .expect("Failed to retrieve all data")
    }

    fn select_by_id(&self, idp: i32) -> Vec<Country> {
        use super::super::schema::country::dsl::*;
        country.
            filter(id.eq(idp))
            .load::<Country>(&self.connection)
            .unwrap_or_else(|_| panic!("Failed to retrieve country {}", idp))
    }
}
