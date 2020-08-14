use chrono::{Duration, NaiveDateTime};
use diesel::prelude::*;

use crate::database::dto;
use crate::database::dto::user::User;
use crate::database::infra::repository::Repository;
use crate::database::repository::user_repository::UsersRepository;
use crate::service::authentication_service::generate_token;

const ONE_HOUR: i64 = 3600;

pub fn refresh(connection: &MysqlConnection, user: &User) -> Result<User, String> {
    let repository = UsersRepository::new(connection);
    let user = repository.select_by_name(user.get_name());

    match user.get(0) {
        Some(user) => Ok(renew_token(&user, resolve_date(*user.get_updated_at()))),
        None => Err(String::from("User not found")),
    }
}

fn renew_token(user: &User, updated_at: NaiveDateTime) -> User {
    //println!(" => real: {} user: {}", updated_at, user.get_updated_at());
    match updated_at.timestamp() - user.get_updated_at().timestamp() {
        0 ..=3600 => User::new(
            *user.get_id(),
            user.get_name().clone(),
            user.get_password().clone(),
            user.get_token().clone(),
            *user.get_role_id(),
            *user.get_created_at(),
            *user.get_updated_at(),
        ),
        _ => User::new(
                *user.get_id(),
                user.get_name().clone(),
                user.get_password().clone(),
                generate_token(),
                *user.get_role_id(),
                *user.get_created_at(),
                *user.get_updated_at(),
            )
    }
    //TODO store user in database
}

fn resolve_date(updated_at: NaiveDateTime) -> NaiveDateTime {
    let now = dto::now();
    //println!("-> {}", updated_at.timestamp_millis() - now.timestamp_millis());
    if updated_at.timestamp_millis() - now.timestamp_millis() <= 0 {
        return now.checked_add_signed(Duration::seconds(ONE_HOUR)).unwrap();
    }
    updated_at
}

#[test]
fn should_renew() {
    use chrono::NaiveDate;

    let token = String::from("thisisatoken039933@@#");
    let user = User::new(
        0,
        String::from("rolfie"),
        String::from("rolfie"),
        token.clone(),
        1,
        NaiveDate::from_ymd(1999, 8, 2).and_hms(15, 30, 12),
        NaiveDate::from_ymd(1999, 8, 5).and_hms(12, 30, 16),
    );

    assert_ne!(
        token,
        renew_token(&user, resolve_date(user.get_updated_at().clone()))
        .get_token()
        .clone()
    );
}

#[test]
fn should_not_renew() {
    use chrono::{Duration, NaiveDate};

    let token = String::from("thisisatoken039933@@#");
    let user = User::new(
        0,
        String::from("rolfie"),
        String::from("rolfie"),
        token.clone(),
        1,
        dto::now(),
        dto::now(),
    );

    assert_eq!(
        token,
        renew_token(&user, resolve_date(user.get_updated_at().clone()))
        .get_token()
        .clone()
    );

    let user = User::new(
        0,
        String::from("rolfie"),
        String::from("rolfie"),
        token.clone(),
        1,
        NaiveDate::from_ymd(1999, 8, 2).and_hms(15, 30, 12),
        dto::now()
        .checked_add_signed(Duration::seconds(3600))
        .unwrap(), //+1hour
    );

    assert_eq!(
        token,
        renew_token(&user, resolve_date(user.get_updated_at().clone()))
        .get_token().clone()
    );
}

