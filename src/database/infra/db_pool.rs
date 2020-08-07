use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

type PoolMysql = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool_mysql(data_base_url: String) -> PoolMysql {
    let manager = ConnectionManager::new(&data_base_url);
    PoolMysql::new(manager)
        .unwrap_or_else(|_| panic!("Failed to create pool with {}", &data_base_url))
}

pub struct DBConnectionMysql(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DBConnectionMysql {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DBConnectionMysql, Self::Error> {
        let pool = request.guard::<State<PoolMysql>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DBConnectionMysql(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DBConnectionMysql {
    type Target = MysqlConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

//For testing
pub fn create_connexion(url: &str) -> MysqlConnection {
        MysqlConnection::establish(url)
            .expect("Expect url database")
}

