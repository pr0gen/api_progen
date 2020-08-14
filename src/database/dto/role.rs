use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::dto::Dto;
use crate::database::schema::role;

#[derive(Serialize, Queryable, Deserialize)]
pub struct Role {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[table_name = "role"]
pub struct InsertableRole {
    name: String,
}

impl Dto for Role {}

impl Role {
    pub fn new(id: i32, name: String) -> Self {
        Role { id, name }
    }
}

impl InsertableRole {
    pub fn from_role(role: &Role) -> Self {
        InsertableRole {
            name: String::from(&role.name),
        }
    }
}
