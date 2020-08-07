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
