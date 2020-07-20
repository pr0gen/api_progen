#![feature(proc_macro_hygiene, decl_macro)]

extern crate chrono;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;

pub mod controller;
pub mod database;
pub mod router;
pub mod service;

fn main() {
    router::creates_all_routes();
}

