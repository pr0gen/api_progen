use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub fn create_connexion(url: &String) -> MysqlConnection {
    MysqlConnection::establish(url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", url))
}


