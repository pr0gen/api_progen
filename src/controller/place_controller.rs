use crate::database::dto::place::Place;
use crate::database::dto::place::PlacesHandler;
use crate::database::infra::connection_builder::{MySQLConnectionBuilder, SQLConnectionBuilder};
use crate::database::infra::entities_handlers::EntityHandler;
use rocket_contrib::json::Json;

use dotenv::dotenv;
use std::env;

#[get("/parking")]
pub fn parking() -> Json<Vec<Place>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection_builder = MySQLConnectionBuilder::new(database_url);
    let place_handler = PlacesHandler::new(connection_builder.create_connexion());
    Json(place_handler.select())
}

