pub mod parking_router;

pub fn creates_all_routes() {
    parking_router::create_routes();
}

use dotenv::dotenv;
use std::env;

pub fn data_base_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
