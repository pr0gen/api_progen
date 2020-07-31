use rocket_contrib::json::Json;

use crate::database::dto::place::Place;
use crate::database::infra::db_pool::DBConnectionMysql;
use crate::service::place_service;


#[get("/all")]
pub fn get_all(connection: DBConnectionMysql) -> Json<Vec<Place>> {
    Json(place_service::select(&*connection))
}

#[get("/by-city/<name>")]
pub fn get_by_city(connection: DBConnectionMysql, name: String) -> Json<Vec<Place>> {
    Json(place_service::select_by_city(&*connection, &name))
}

