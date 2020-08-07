use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::dto::Dto;
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


impl Dto for Country {}

impl Country {
    pub fn new(id: i32, name: String) -> Self {
        Country { id, name }
    }
}

impl InsertableCountry {
    pub fn from_country(country: &Country) -> InsertableCountry {
        InsertableCountry {
            name: country.name.to_string(),
        }
    }
}

