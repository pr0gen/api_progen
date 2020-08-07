use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::dto::Dto;
use crate::database::schema::city;

#[derive(Serialize, Queryable, Deserialize)]
pub struct City {
    id: i32,
    name: String,
    postal_code: i32,
    country_id: i32,
}

#[derive(Insertable)]
#[table_name = "city"]
pub struct InsertableCity {
    name: String,
    postal_code: i32,
    country_id: i32,
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
    pub fn from_city(city: &City) -> InsertableCity {
        InsertableCity {
            name: city.name.to_string(),
            postal_code: city.postal_code,
            country_id: city.country_id,
        }
    }
}

