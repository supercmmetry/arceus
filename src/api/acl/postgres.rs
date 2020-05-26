use diesel::prelude::*;
use crate::api::acl::repository::Repository;
use crate::api::entities::user::User;
use crate::utils::errors::Error;

pub struct PostgresRepo<'a> {
    connection: &'a PgConnection
}

impl<'a> PostgresRepo<'a> {
    pub fn new(connection: &'a PgConnection) -> Self {
        PostgresRepo {
            connection
        }
    }
}

impl<'a> Repository for PostgresRepo<'a> {
    fn add_user(user: &User) -> Result<(), Error> {
        unimplemented!()
    }

    fn get_users() -> Result<Vec<User>, Error> {
        unimplemented!()
    }

    fn remove_user(username: String) -> Result<(), Error> {
        unimplemented!()
    }

    fn demote_user(username: String) -> Result<(), Error> {
        unimplemented!()
    }

    fn promote_user(username: String) -> Result<(), Error> {
        unimplemented!()
    }
}