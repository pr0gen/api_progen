use super::super::controller::place_controller;

use super::super::database::infra::db_pool;

pub fn create_routes() {
    //TODO Add call DBPool here
    rocket::ignite()
        .manage(db_pool::init_pool())
        .mount(
            "/parking",
            routes![
                place_controller::parking
            ],
        )
        .launch();
}

