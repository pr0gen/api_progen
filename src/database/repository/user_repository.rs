use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Connection, MysqlConnection};

use crate::database::dto::user::{User, InsertableUser};
use crate::database::infra::repository::Repository;
use crate::database::schema::user;

pub struct UsersRepository<'a, C: Connection> {
    connection: &'a C,
}

impl<'a> Repository<'a, MysqlConnection, User> for UsersRepository<'a, MysqlConnection> {
    fn new(connection: &'a MysqlConnection) -> Self {
        UsersRepository { connection }
    }

    fn select(&self) -> Vec<User> {
        use super::super::schema::user::dsl::*;
        user.load::<User>(self.connection)
            .expect("Failed to retrieve all data")
    }

    fn select_by_id(&self, idp: i32) -> Vec<User> {
        use super::super::schema::user::dsl::*;
        user.filter(id.eq(idp))
            .load::<User>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to retrieve user {}", idp))
    }

    fn insert(&self, data: &User) -> QueryResult<usize> {
        diesel::insert_into(user::table)
            .values(&InsertableUser::from_user(data))
            .execute(self.connection)
    }

    fn insert_multiples(&self, data: &[User]) -> QueryResult<usize> {
        let insert_users: Vec<InsertableUser> = data
            .iter()
            .map(|city| InsertableUser::from_user(city))
            .collect();
        diesel::insert_into(user::table)
            .values(insert_users)
            .execute(self.connection)
    }
}

impl<'a> UsersRepository<'a, MysqlConnection> {
    pub fn select_by_name(&self, name_user: &str) -> Vec<User> {
        use crate::database::schema::user::dsl::*;
        user.filter(name.eq(name_user))
            .load::<User>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to find user {}", name_user))
    }

    //TODO adding to trait
    //I would like to be able to use exist, but with mysql,
    //I need to use .execute() which is return the number of affected rows and
    //it is not useful in this case, gl hf mate
    //Perhaps I will refactor this thing later
    pub fn exists(&self, name_user: &str) -> Result<bool, Error> {
        let result = self.select_by_name(&name_user);
        match result.len() {
            0 => Ok(false),
            _ => Ok(true),
        }
    }
}

#[test]
fn should_insert_and_select() {
    use crate::database::infra::db_pool;
    use crate::router;
    use crate::database::dto::user::as_user;
    let to_insert = as_user(String::from("malokran"), String::from("malokran"), 1);
    let connection = db_pool::create_connexion(router::test_data_base_url().as_str());
    connection.test_transaction::<_, Error, _>(|| {
        let repository = UsersRepository::new(&connection);
        repository.insert(&to_insert)?;

        use crate::database::schema::user::table;
        let all = table.select(user::name)
            .load::<String>(&connection)?;

        assert!(all.contains(&String::from("malokran")));
        Ok(())
    });
}

