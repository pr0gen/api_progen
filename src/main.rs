#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod database;
pub mod controller;
pub mod service;
pub mod router;

fn main() {
    router::creates_all_routes();
}
