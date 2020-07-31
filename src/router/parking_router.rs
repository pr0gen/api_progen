use crate::controller::place_controller::*;
use crate::database::infra::db_pool;
use super::data_base_url;


pub fn create_routes() {
    rocket::ignite()
        .manage(db_pool::init_pool_mysql(data_base_url()))
            
        .mount(
            "/place",
            routes![
                get_all,
                get_by_city
            ],
        )
        .launch();
}

