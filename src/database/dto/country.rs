use diesel::prelude::*;
use diesel::{Connection, Insertable, MysqlConnection, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::dto::Dto;
use crate::database::infra::repository::Repository;
use crate::database::schema::country;

#[derive(Serialize, Queryable, Deserialize)]
pub struct Country {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[table_name = "country"]
pub struct InsertableCountry {
    name: String,
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

impl InsertableCountry {
    fn from_country(country: &Country) -> InsertableCountry {
        InsertableCountry {
            name: country.name.to_string(),
        }
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

    fn insert(&self, data: &Country) -> QueryResult<usize> {
        diesel::insert_into(country::table)
            .values(&InsertableCountry::from_country(data))
            .execute(self.connection)
    }

    fn insert_multiples(&self, data: &[Country]) -> QueryResult<usize> {
        let insert_countries: Vec<InsertableCountry> = data
            .iter()
            .map(|country| InsertableCountry::from_country(country))
            .collect();
        diesel::insert_into(country::table)
            .values(insert_countries)
            .execute(self.connection)
    }
}
