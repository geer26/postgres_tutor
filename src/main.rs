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
    //println!("Hello, world!");
    //let _conn: PgConnection = establish_connection();
    //println!("{:?}",&_conn.type_id());

    let mut gergo = User::new("Gergo".to_string());
    gergo.set_password("123456aA".to_string());

    println!("PASSWORD CHECK: {}", gergo.check_password("123456aA".to_string()));
    //println!("PW HASH:{}", &gergo.password_hash);
    //println!("SALT: {}", &gergo.salt)
}
