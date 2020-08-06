use argon2::{self, Config};
use rand::distributions::Alphanumeric;
use rand::Rng;
use diesel::prelude::*;

use crate::database::infra::repository::Repository;
use crate::database::dto::user::UsersRepository;
use crate::database::dto::user::User;

pub fn register(connection: &MysqlConnection, user: &User) -> QueryResult<usize> {
    UsersRepository::new(connection)
        .insert(user)
}

pub fn login(connection: &MysqlConnection, user_log: &User) -> Result<String, String> {
    let user_db = UsersRepository::new(connection)
        .select_by_name(user_log.get_name());

    match user_db.get(0) {
        Some(user) => {
            if verify_password(user.get_password(), user_log.get_password()){
                return Ok(generate_token());
            }
            Err(String::from("Failed to verify password"))
        },
        None => Err(String::from("User not found")),
    }
}

fn generate_token() -> String {
    generate_string(255)    
}

pub fn hash_password(password: &str) -> String {
    argon2::hash_encoded(password.as_bytes(), salt().as_bytes(), &config())
        .unwrap_or_else(|_| panic!("Failed to hash password"))
}

pub fn verify_password(hashed_password: &str, password: &str) -> bool {
    argon2::verify_encoded(hashed_password, password.as_bytes())
        .unwrap()
}

fn salt() -> String {
    generate_string(25)
}

fn generate_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect::<String>()
}

fn config<'a>() -> Config<'a> {
    Config::default()
}
