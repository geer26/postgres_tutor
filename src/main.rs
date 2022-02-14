#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use std::any::Any;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use crate::models::User;
//use std::hash::Hash;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}



fn main() {
    println!("Hello, world!");
    let _conn: PgConnection = establish_connection();
    println!("{:?}",&_conn.type_id());

    let gergo = User::new(User, "Gergo".to_string(), "password".to_string());
}
