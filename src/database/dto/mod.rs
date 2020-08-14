use chrono::NaiveDateTime;
use chrono::Utc;

pub mod city;
pub mod country;
pub mod place;
pub mod role;
pub mod user;

pub trait Dto {}

pub fn now() -> NaiveDateTime {
    Utc::now().naive_utc()
}
