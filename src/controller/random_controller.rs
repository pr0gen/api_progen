use rocket_contrib::json::Json;
use crate::service::random_service::RandomCityService;
use crate::database::dto::place::Place;


#[get("/random")]
pub fn random_parking() -> Json<Vec<Place>> {
    let random_service = RandomCityService::new();
    Json(random_service.get_random_parking("paris"))
}
