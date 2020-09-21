pub mod login_controller;
pub mod place_controller;

#[derive(Debug)]
pub enum APIProgenError {
    RegisterError(String),
    LoginError(String),
}

impl APIProgenError {

    pub fn message(&self) -> String {
        match self {
            APIProgenError::RegisterError(s) => s.to_owned(),
            APIProgenError::LoginError(s) => s.to_owned(),
        }
    }
}

