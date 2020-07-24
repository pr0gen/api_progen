use super::super::controller::place_controller;
use super::super::database::infra::db_pool;
use super::data_base_url;


pub fn create_routes() {
    rocket::ignite()
        .manage(db_pool::init_pool(data_base_url()))
        .mount(
            "/parking",
            routes![
                place_controller::parking
            ],
        )
        .launch();
}

