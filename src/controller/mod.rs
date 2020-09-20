pub mod login_controller;
pub mod place_controller;

#[derive(Debug)]
pub enum APIProgenError {
    RegisterError(String),
}



