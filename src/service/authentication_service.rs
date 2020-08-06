use argon2::{self, Config};
use diesel::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::database::dto::user::User;
use crate::database::dto::user::UsersRepository;
use crate::database::infra::repository::Repository;

pub fn register(connection: &MysqlConnection, user: &User) -> Result<String, String> {
    let hashed_user = User::new(
        *user.get_id(),
        user.get_name().clone(),
        hash_password(user.get_password()),
        generate_token(),
        *user.get_role_id(),
    );
    let repository = UsersRepository::new(connection);

    if repository.exists(user.get_name()) {
        return Err(String::from("Already exists !"));
    }

    let result = repository.insert(&hashed_user);
    if result.is_err() {
        return Err(format!("Failed to insert user {} in database", user.get_name()));
    }

    Ok(String::from("Successfuly register user"))
}

pub fn login(connection: &MysqlConnection, user_log: &User) -> Result<String, String> {
    let user_db = UsersRepository::new(connection).select_by_name(user_log.get_name());

    match user_db.get(0) {
        Some(user) => {
            if verify_password(user.get_password(), user_log.get_password()) {
                return Ok(generate_token());
            }
            Err(String::from("Failed to verify password"))
        }
        None => Err(String::from("User not found")),
    }
}

fn generate_token() -> String {
    generate_string(255)
}

#[test]
fn should_generate_token() {
    assert_eq!(generate_token().len(), 255);
}

pub fn hash_password(password: &str) -> String {
    argon2::hash_encoded(password.as_bytes(), salt().as_bytes(), &config())
        .unwrap_or_else(|_| String::from("Failed to hash password"))
}

pub fn verify_password(hashed_password: &str, password: &str) -> bool {
    argon2::verify_encoded(hashed_password, password.as_bytes()).unwrap()
}

#[test]
fn should_hash_and_check_decode() {
    let hash = hash_password("bonjour");
    assert!(verify_password(hash.as_str(), "bonjour"))
}

fn salt() -> String {
    generate_string(25)
}

#[test]
fn should_generate_salt() {
    assert_eq!(salt().len(), 25);
}

fn generate_string(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .collect::<String>()
}

#[test]
fn should_generate_string() {
    assert_eq!(generate_string(10).len(), 10);
}

fn config<'a>() -> Config<'a> {
    Config::default()
}
