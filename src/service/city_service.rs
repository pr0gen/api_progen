use crate::database::repository::city_repository::CitiesRepository;
use crate::database::dto::city::City;
use crate::database::infra::repository::Repository;
use diesel::mysql::MysqlConnection;

pub fn select(connection: &MysqlConnection) -> Vec<City> {
    CitiesRepository::new(connection).select()
}

pub fn select_by_name(connection: &MysqlConnection, city_name: &str) -> Vec<City> {
    CitiesRepository::new(connection).select_by_name(city_name)
}
