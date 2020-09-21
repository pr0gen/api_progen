use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::dto::user::as_user;
use crate::database::dto::user::User;
use crate::database::infra::db_pool::DBConnectionMysql;
use crate::service::authentication_service;

#[post("/register", format = "application/json", data = "<user>")]
pub fn register(connection: DBConnectionMysql, user: Json<JsonUser>) -> Json<(User, String)> {
    let user = user.0;
    let user: User = as_user(user.name, user.password, user.role_id);
    match authentication_service::register(&*connection, &user) {
        Ok(registered_user) => Json((registered_user, String::from(""))),
        Err(e) => Json((user, e.message())),
    }
}

#[post("/", format = "application/json", data = "<user>")]
pub fn login(connection: DBConnectionMysql, user: Json<JsonUser>) -> String {
    let user = user.0;
    let token = &user.token;
    if !token.is_empty() {
        return check_token(connection, token);
    }
    let user: User = as_user(user.name, user.password, user.role_id);
    authentication_service::login(&*connection, &user)
        .unwrap_or_else(|e| e.message())
}

fn check_token(connection: DBConnectionMysql, token: &str) -> String {
    match authentication_service::check_token(&*connection, token) {
        Ok(true) => String::from("Successfuly checked token !"), //TODO add token refresher
        Ok(false) => String::from("Failed to check token :("),
        Err(_e) => String::from("Something went wrong"),
    }
}

#[derive(Serialize, Deserialize)]
pub struct JsonUser {
    name: String,
    password: String,
    token: String,
    role_id: i32,
}

