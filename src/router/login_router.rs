use crate::controller::login_controller::*;
use rocket::Route;

pub fn create_routes_login() -> Vec<Route> {
    routes![register, login]
}
