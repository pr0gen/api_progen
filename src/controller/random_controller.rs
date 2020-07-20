use crate::database::dto::place::Place;
use crate::service::random_service::RandomCityService;
use rocket_contrib::json::Json;

#[get("/random")]
pub fn random_parking() -> Json<Vec<Place>> {
    let random_service = RandomCityService::default();
    Json(random_service.get_random_parking("paris"))
}

