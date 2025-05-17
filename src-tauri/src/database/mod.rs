use diesel::{Connection, SqliteConnection};
use dotenv::dotenv;
use std::env;

pub mod file_scanner;
pub mod music_database;
pub mod settings_database;

pub fn open_database_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
