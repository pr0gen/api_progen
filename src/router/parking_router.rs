use super::super::controller::place_controller;
use super::super::controller::random_controller;

pub fn create_routes() {
    rocket::ignite()
        .mount(
            "/parking",
            routes![random_controller::random_parking, place_controller::parking],
        )
        .launch();
}

