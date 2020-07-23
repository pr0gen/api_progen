use crate::database::dto::place::Place;
use crate::database::dto::place::PlacesRepository;
use crate::database::infra::repository::Repository;

use rocket_contrib::json::Json;

use super::super::database::infra::db_pool::DBConnection;

#[get("/parking")]
pub fn parking(connection: DBConnection) -> Json<Vec<Place>> {
    let place_handler = PlacesRepository::new(&*connection);
    Json(place_handler.select())
}

