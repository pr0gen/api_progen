use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use rocket::{Outcome, Request, State};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use std::ops::Deref;

type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool(data_base_url: String) -> Pool {
    let manager = ConnectionManager::new(&data_base_url);
    Pool::new(manager)
        .unwrap_or_else(|_| panic!("Failed to create pool with {}", &data_base_url))
}

pub struct DBConnection(pub r2d2::PooledConnection<ConnectionManager<MysqlConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DBConnection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<DBConnection, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DBConnection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DBConnection {
    type Target = MysqlConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

