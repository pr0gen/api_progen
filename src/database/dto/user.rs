use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::dto;
use crate::database::dto::Dto;
use crate::database::schema::user;

pub fn as_user(name: String, password: String, role_id: i32) -> User {
    User {
        id: 0,
        name,
        password,
        token: String::new(),
        role_id,
        created_at: dto::now(),
        updated_at: dto::now(),
    }
}

#[test]
pub fn should_convert() {
    let expected = User::new(
        0,
        String::from("rolfie"),
        String::from("rolfie"),
        String::from(""),
        1,
        dto::now(),
        dto::now(),
    );
    let actual = as_user(String::from("rolfie"), String::from("rolfie"), 1);
    assert_eq!(actual.name, expected.name);
    assert_eq!(actual.password, expected.password);
    assert_eq!(actual.token, expected.token);
}

#[derive(Serialize, Queryable, Deserialize, Debug)]
pub struct User {
    id: i32,
    name: String,
    password: String,
    token: String,
    role_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct InsertableUser {
    name: String,
    password: String,
    token: String,
    role_id: i32,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Dto for User {}

impl User {
    pub fn new(
        id: i32,
        name: String,
        password: String,
        token: String,
        role_id: i32,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        User {
            id,
            name,
            password,
            token,
            role_id,
            created_at,
            updated_at,
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

    pub fn get_updated_at(&self) -> &NaiveDateTime {
        &self.updated_at
    }

    pub fn get_created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}

impl InsertableUser {
    pub fn from_user(user: &User) -> Self {
        InsertableUser {
            name: String::from(&user.name),
            password: String::from(&user.password),
            token: String::from(&user.token),
            role_id: user.role_id,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}
