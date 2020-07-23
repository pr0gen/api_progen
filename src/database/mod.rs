pub mod dto;
pub mod infra;
pub mod schema;

use dotenv::dotenv;
use std::env;

pub fn data_base_url() -> String {
    dotenv().ok();
    env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
}

