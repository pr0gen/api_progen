use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::dto::Dto;
use crate::database::schema::user;

pub fn as_user(name: String, password: String, role_id: i32) -> User {
    User {
        id: 0,
        name,
        password,
        token: String::new(),
        role_id,
    }
}

#[derive(Serialize, Queryable, Deserialize, Debug)]
pub struct User {
    id: i32,
    name: String,
    password: String,
    token: String,
    role_id: i32,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct InsertableUser {
    name: String,
    password: String,
    token: String,
    role_id: i32,
}

impl Dto for User {}

impl User {
    pub fn new(id: i32, name: String, password: String, token: String, role_id: i32) -> Self {
        User {
            id,
            name,
            password,
            token,
            role_id,
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }

    pub fn get_token(&self) -> &String {
        &self.token
    }

    pub fn get_role_id(&self) -> &i32 {
        &self.role_id
    }
}

impl InsertableUser {
    pub fn from_user(user: &User) -> Self {
        InsertableUser {
            name: String::from(&user.name),
            password: String::from(&user.password),
            token: String::from(&user.token),
            role_id: user.role_id,
        }
    }
}

