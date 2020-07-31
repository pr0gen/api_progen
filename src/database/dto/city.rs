use diesel::prelude::*;
use diesel::{Connection, Queryable, Insertable};
use serde::{Deserialize, Serialize};

use crate::database::schema::city;
use crate::database::dto::Dto;
use crate::database::infra::repository::Repository;

#[derive(Serialize, Queryable, Deserialize)]
pub struct City {
    id: i32,
    name: String,
    postal_code: i32,
    country_id: i32,
}

#[derive(Insertable)]
#[table_name ="city"]
pub struct InsertableCity {
    name: String,
    postal_code: i32,
    country_id: i32,
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

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

impl InsertableCity {

    fn from_city(city: City) -> InsertableCity {
        InsertableCity {
            name: city.name,
            postal_code: city.postal_code,
            country_id: city.country_id,
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
            .unwrap_or_else(|_| panic!("Failed to retrieve city {}", idp))
    }

    fn insert(&self, data: City) -> QueryResult<usize> {
        diesel::insert_into(city::table)
            .values(&InsertableCity::from_city(data))
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

