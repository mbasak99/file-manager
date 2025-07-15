// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

fn create_sql_conn() -> SqliteConnection {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    SqliteConnection::establish(&db_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", db_url))
}

fn main() {
    file_manager_lib::run();

    // create SQLite connection
    create_sql_conn();
}
