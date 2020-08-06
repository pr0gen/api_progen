use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::dto::user::as_user;
use crate::database::dto::user::User;
use crate::database::infra::db_pool::DBConnectionMysql;
use crate::service::authentication_service;

#[post("/register", format = "application/json", data = "<user>")]
pub fn register(connection: DBConnectionMysql, user: Json<JsonUser>) -> String {
    let user = user.0;
    let user: User = as_user(user.name, user.password, user.role_id);
    authentication_service::register(&*connection, &user)
        .unwrap()
}

#[post("/", format = "application/json", data = "<user>")]
pub fn login(connection: DBConnectionMysql, user: Json<JsonUser>) -> String {
    let user = user.0;
    let user: User = as_user(user.name, user.password, user.role_id);
    authentication_service::login(&*connection, &user)
        .unwrap_or_else(|_| String::from("Failed to check in user"))
}

#[derive(Serialize, Deserialize)]
pub struct JsonUser {
    name: String,
    password: String,
    //token: String,
    role_id: i32,
}
