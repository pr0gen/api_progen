use diesel::prelude::*;
use diesel::{Connection, Insertable, MysqlConnection, Queryable};
use diesel::dsl::exists;
use serde::{Deserialize, Serialize};

use crate::database::dto::Dto;
use crate::database::infra::repository::Repository;
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
struct InsertableUser {
    name: String,
    password: String,
    token: String,
    role_id: i32,
}

pub struct UsersRepository<'a, C: Connection> {
    connection: &'a C,
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

impl<'a> Repository<'a, MysqlConnection, User> for UsersRepository<'a, MysqlConnection> {
    fn new(connection: &'a MysqlConnection) -> Self {
        UsersRepository { connection }
    }

    fn select(&self) -> Vec<User> {
        use super::super::schema::user::dsl::*;
        user.load::<User>(self.connection)
            .expect("Failed to retrieve all data")
    }

    fn select_by_id(&self, idp: i32) -> Vec<User> {
        use super::super::schema::user::dsl::*;
        user.filter(id.eq(idp))
            .load::<User>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to retrieve user {}", idp))
    }

    fn insert(&self, data: &User) -> QueryResult<usize> {
        diesel::insert_into(user::table)
            .values(&InsertableUser::from_user(data))
            .execute(self.connection)
    }

    fn insert_multiples(&self, data: &[User]) -> QueryResult<usize> {
        let insert_users: Vec<InsertableUser> = data
            .iter()
            .map(|city| InsertableUser::from_user(city))
            .collect();
        diesel::insert_into(user::table)
            .values(insert_users)
            .execute(self.connection)
    }
}

impl<'a> UsersRepository<'a, MysqlConnection> {
    pub fn select_by_name(&self, name_user: &str) -> Vec<User> {
        use crate::database::schema::user::dsl::*;
        user.filter(name.eq(name_user))
            .load::<User>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to find user {}", name_user))
    }

    //TODO adding to trait
    pub fn exists(&self, name_user: &str) -> bool {
        use crate::database::schema::user::dsl::*;
        let result = diesel::select(exists(user.filter(name.eq(name_user))))
            .execute(self.connection);
        if result.unwrap() == 1 {
            return true;
        }
        false
    }
}
