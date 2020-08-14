use rocket::Route;

use crate::controller::place_controller::*;

pub fn create_routes_place() -> Vec<Route> {
    routes![get_all, get_by_city, add, add_multiples]
}
