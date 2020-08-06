use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::infra::db_pool::DBConnectionMysql;
use crate::service::authentication_service;
use crate::database::dto::user::as_user;
use crate::database::dto::user::User;

#[post("/register", format = "application/json", data = "<user>")]
pub fn register(connection: DBConnectionMysql, user: Json<JsonUser>) -> String {
    let user = user.0;
    let user: User = as_user(user.name, user.password, user.token, user.role_id);
    let result = authentication_service::register(&*connection, &user);
    if result.is_err() {
        return format!(
            "Failed to insert user {} in database",
            user.get_name()
        );
    }
    String::from("Successfuly register user")
}

#[get("/login")]
pub fn login() -> String {
    unimplemented!()
}

#[derive(Serialize, Deserialize)]
pub struct JsonUser {
    name: String,
    password: String,
    token: String,
    role_id: i32,
}


