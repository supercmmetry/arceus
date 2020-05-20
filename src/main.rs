#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket::Request;
use rocket_contrib::json::Json;
use crate::middleware::auth::{Claims, ClaimResult};
use std::env;
use crate::utils::errors::Error;

mod middleware;
mod utils;


#[derive(Deserialize)]
struct Text {
    text: String
}

#[post("/", format = "json", data = "<text>")]
fn index(text: Json<Text>) -> String {
    Claims::new("Vishaal Selvaraj".to_string()).jwt().unwrap()
}

#[post("/auth")]
fn test_auth(claims: ClaimResult) -> ClaimResult {
    claims
}


fn main() {
    dotenv::dotenv().ok();

    rocket::ignite().mount("/", routes![index, test_auth]).launch();
}
