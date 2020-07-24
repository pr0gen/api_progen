use crate::database::dto::Dto;
use crate::database::infra::repository::Repository;
use diesel::prelude::*;
use diesel::{Connection, MysqlConnection, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Queryable, Deserialize)]
pub struct Country {
    pub id: i32,
    pub name: String,
}

pub struct CountriesRepository<'a, C: Connection> {
    connection: &'a C,
}

impl Dto for Country {}

impl Country {
    pub fn new(id: i32, name: String) -> Self {
        Country { id, name }
    }
}

impl<'a> Repository<'a, MysqlConnection, Country> for CountriesRepository<'a, MysqlConnection> {
    fn new(connection: &'a MysqlConnection) -> Self {
        CountriesRepository { connection }
    }

    fn select(&self) -> Vec<Country> {
        use super::super::schema::country::dsl::*;
        country
            .load::<Country>(self.connection)
            .expect("Failed to retrieve all data")
    }

    fn select_by_id(&self, idp: i32) -> Vec<Country> {
        use super::super::schema::country::dsl::*;
        country
            .filter(id.eq(idp))
            .load::<Country>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to retrieve country {}", idp))
    }

    fn insert(&self, data: impl Dto) {
        
    }
}

