use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

use crate::database::dto::place::Place;
use crate::database::dto::place::PlacesRepository;
use crate::database::infra::repository::Repository;
use crate::service::city_service;

pub fn select(connection: &MysqlConnection) -> Vec<Place> {
    PlacesRepository::new(connection).select()
}

pub fn select_by_id(connection: &MysqlConnection, id: i32) -> Vec<Place> {
    PlacesRepository::new(connection).select_by_id(id)
}

pub fn select_by_city(connection: &MysqlConnection, city_name: &str) -> Vec<Place> {
    let city = city_service::select_by_name(connection, city_name);
    let city = city.get(0).unwrap_or_else(|| panic!("Failed to get city"));
    PlacesRepository::new(connection).select_by_city(city)
}

pub fn add(connection: &MysqlConnection, place: &Place) -> QueryResult<usize> {
    PlacesRepository::new(connection).insert(place)
}

pub fn add_multiples(connection: &MysqlConnection, places: &[Place]) -> QueryResult<usize> {
    PlacesRepository::new(connection).insert_multiples(places)
}
