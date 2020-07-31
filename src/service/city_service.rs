use diesel::mysql::MysqlConnection;
use crate::database::dto::city::City;
use crate::database::dto::city::CitiesRepository;
use crate::database::infra::repository::Repository;

pub fn select(connection: &MysqlConnection) -> Vec<City> {
    CitiesRepository::new(connection)
        .select()
}

pub fn select_by_name(connection: &MysqlConnection, city_name: &String) -> Vec<City> {
    CitiesRepository::new(connection)
        .select_by_name(city_name)
}

