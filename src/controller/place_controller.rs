use crate::database::dto::place::Place;
use crate::database::infra::db_pool::DBConnection;
use crate::service::place_service;

use rocket_contrib::json::Json;

#[get("/parking")]
pub fn parking(connection: DBConnection) -> Json<Vec<Place>> {
    Json(place_service::select(connection))
}

