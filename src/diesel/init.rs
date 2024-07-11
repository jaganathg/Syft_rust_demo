use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Database must be set");
    SqliteConnection::establish(&db_url).unwrap_or_else(|_| {
        panic!("Failed to establish connection to database")
    })
}


fn main() {
    establish_connection();
    println!("Connection established!")
}