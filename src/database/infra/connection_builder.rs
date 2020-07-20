use diesel::mysql::MysqlConnection;
use diesel::prelude::*;

pub trait SQLConnectionBuilder<C: Connection> {
    fn create_connexion(&self) -> C;
}

pub struct MySQLConnectionBuilder {
    url: String
}

impl MySQLConnectionBuilder {
    pub fn new(url: String) -> Self {
        return MySQLConnectionBuilder {
            url
        };
    }

    pub fn from_fields(username: String, password: String, host: String, name: String) -> Self {
        return MySQLConnectionBuilder {
            url: format!("{}://{}:{}@{}/{}", String::from("mysql"), username, password, host, name)
        };
    }
}

impl SQLConnectionBuilder<MysqlConnection> for MySQLConnectionBuilder {
    fn create_connexion(&self) -> MysqlConnection {
        return MysqlConnection::establish(&self.url)
            .expect(&format!("Error connecting to {}", self.url));
    }
}
