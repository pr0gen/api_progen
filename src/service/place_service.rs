use crate::database::dto::place::Place;
use crate::database::dto::place::PlacesRepository;
use crate::database::infra::repository::Repository;
use crate::database::infra::db_pool::DBConnection;

pub fn select(connection: DBConnection) -> Vec<Place> {
    PlacesRepository::new(&*connection)
        .select()
}

pub fn select_by_id(connection: DBConnection, id: i32) -> Vec<Place> {
    PlacesRepository::new(&*connection)
        .select_by_id(id)
}


