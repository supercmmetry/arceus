#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde;
#[macro_use]
extern crate diesel;

use diesel::{PgConnection, Connection};
use std::env;
use crate::utils::errors::{Error, ErrorCode};

mod api;
mod middleware;
mod utils;


fn connect_to_db() -> Result<PgConnection, Error> {
    let mut db_url: String = String::default();
    match env::var("POSTGRES_DB_URL") {
        Ok(url) => db_url = url,
        Err(e) => return Err(Error::custom(ErrorCode::DatabaseError, e.to_string()))
    };

    match PgConnection::establish(db_url.as_str()) {
        Ok(conn) => Ok(conn),
        Err(e) => Err(Error::custom(ErrorCode::DatabaseError, e.to_string()))
    }
}

fn main() {
    dotenv::dotenv().ok();
    let res = connect_to_db();
}
