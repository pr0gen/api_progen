use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::database::dto::place;
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

#[post("/add", format = "application/json", data = "<place>")]
pub fn add(connection: DBConnectionMysql, place: Json<JsonPlace>) -> String {
    let place = place.0;
    let place: Place = place::as_place(
        place.longitude,
        place.latitude,
        place.city_id,
        place.nb_place,
    );
    let result = place_service::add(&*connection, &place);
    if result.is_err() {
        return format!(
            "Failed to insert {} place ({}, {}) in database",
            place.get_nb_place(),
            place.get_longitude(),
            place.get_latitude()
        );
    }
    String::from("Successfuly insert place")
}

#[post("/add_multiples", format = "application/json", data = "<places>")]
pub fn add_multiples(connection: DBConnectionMysql, places: Json<Vec<JsonPlace>>) -> String {
    let places = places.0;
    let places: Vec<Place> = places
        .iter()
        .map(|place| {
            place::as_place(
                place.longitude,
                place.latitude,
                place.city_id,
                place.nb_place,
            )
        })
        .collect();
    let result = place_service::add_multiples(&*connection, &places);
    if result.is_err() {
        return String::from("Failed to insert place in database");
    }
    String::from("Successfuly insert places")
}

#[derive(Serialize, Deserialize)]
pub struct JsonPlace {
    longitude: f32,
    latitude: f32,
    city_id: i32,
    nb_place: i32,
}
