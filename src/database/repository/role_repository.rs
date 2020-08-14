use diesel::prelude::*;
use diesel::{Connection, MysqlConnection};

use crate::database::dto::role::{InsertableRole, Role};
use crate::database::infra::repository::Repository;
use crate::database::schema::role;

pub struct RolesRepository<'a, C: Connection> {
    connection: &'a C,
}

impl<'a> Repository<'a, MysqlConnection, Role> for RolesRepository<'a, MysqlConnection> {
    fn new(connection: &'a MysqlConnection) -> Self {
        RolesRepository { connection }
    }

    fn select(&self) -> Vec<Role> {
        use super::super::schema::role::dsl::*;
        role.load::<Role>(self.connection)
            .expect("Failed to retrieve all data")
    }

    fn select_by_id(&self, idp: i32) -> Vec<Role> {
        use super::super::schema::role::dsl::*;
        role.filter(id.eq(idp))
            .load::<Role>(self.connection)
            .unwrap_or_else(|_| panic!("Failed to retrieve role:{}", idp))
    }

    fn insert(&self, data: &Role) -> QueryResult<usize> {
        diesel::insert_into(role::table)
            .values(&InsertableRole::from_role(data))
            .execute(self.connection)
    }

    fn insert_multiples(&self, data: &[Role]) -> QueryResult<usize> {
        let insert_roles: Vec<InsertableRole> = data
            .iter()
            .map(|city| InsertableRole::from_role(city))
            .collect();
        diesel::insert_into(role::table)
            .values(insert_roles)
            .execute(self.connection)
    }
}

#[test]
fn should_insert_and_select() {
    use crate::database::dto::role::Role;
    use crate::database::infra::db_pool;
    use crate::router;
    use diesel::result::Error;
    let to_insert = Role::new(1, String::from("dev"));
    let connection = db_pool::create_connexion(router::test_data_base_url().as_str());
    connection.test_transaction::<_, Error, _>(|| {
        let repository = RolesRepository::new(&connection);
        repository.insert(&to_insert)?;

        use crate::database::schema::role::table;
        let all = table.select(role::name).load::<String>(&connection)?;

        assert!(all.contains(&String::from("dev")));
        Ok(())
    });
}
