pub mod login_router;
pub mod parking_router;

use dotenv::dotenv;
use std::env;

use crate::database::infra::db_pool;

pub fn creates_all_routes() {
    //I should refactor this
    rocket::ignite()
        .manage(db_pool::init_pool_mysql(data_base_url()))
        .mount("/place", parking_router::create_routes_place())
        .mount("/login", login_router::create_routes_login())
        .launch();
}

pub fn data_base_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

pub fn test_data_base_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .or_else(|_| env::var("REMOTE_DATABASE_URL"))
        .expect("Either DATABASE_URL or REMOTE_DATABASE_URL must be set")
}
