use diesel::prelude::*;
use diesel::{Connection, Insertable, MysqlConnection, Queryable};
use serde::{Deserialize, Serialize};

use crate::database::dto::Dto;
use crate::database::infra::repository::Repository;
use crate::database::schema::role;

#[derive(Serialize, Queryable, Deserialize)]
pub struct Role {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[table_name = "role"]
struct InsertableRole {
    name: String,
}

pub struct RolesRepository<'a, C: Connection> {
    connection: &'a C,
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

impl<'a> Repository<'a, MysqlConnection, Role> for RolesRepository<'a, MysqlConnection> {
    fn new(connection: &'a MysqlConnection) -> Self {
        RolesRepository { connection }
    }

    fn select(&self) -> Vec<Role> {
        use super::super::schema::role::dsl::*;
        role.load::<Role>(self.connection)
            .expect("Failed to retrieve all data")
    }

    fn select_by_id(&self, idp: i32) -> Vec<Role> {
        use super::super::schema::role::dsl::*;
        role.filter(id.eq(idp))
            .load::<Role>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to retrieve role:{}", idp))
    }

    fn insert(&self, data: &Role) -> QueryResult<usize> {
        diesel::insert_into(role::table)
            .values(&InsertableRole::from_role(data))
            .execute(self.connection)
    }

    fn insert_multiples(&self, data: &[Role]) -> QueryResult<usize> {
        let insert_roles: Vec<InsertableRole> = data
            .iter()
            .map(|city| InsertableRole::from_role(city))
            .collect();
        diesel::insert_into(role::table)
            .values(insert_roles)
            .execute(self.connection)
    }
}
