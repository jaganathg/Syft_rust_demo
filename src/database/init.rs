use sqlx::{query, SqlitePool};
use sqlx::{sqlite::SqlitePoolOptions, Error};
use std::fs;
use std::path::Path;
use tokio;

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Error> {
    run().await
}

pub async fn initialize_db(db_url: &str) -> Result<SqlitePool, sqlx::Error> {
    // create database if not exist
    let db_path = Path::new(db_url);
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            println!("Creating directory: {:?}", parent);
            fs::create_dir_all(parent).expect("Failed to create db directory");
        } else {
            println!("Directory already exists: {:?}", parent);
        }

        if fs::metadata(parent)
            .expect("Failed to get metadata")
            .permissions()
            .readonly()
        {
            println!("Directory is read-only: {:?}", parent);
        } else {
            println!("Directory is writable: {:?}", parent);
        }
    } else {
        println!("Parent directory not found: {:?}", db_path);
    }

    // Debug print for db_url
    println!("Database URL: {:?}", db_url);

    // connect pool
    let pool = SqlitePool::connect(db_url).await?;
    println!("Database connected successfully!");
    // read sql file
    let init_script = fs::read_to_string("db/init_db.sql").expect("Failed to read init_db.sql");
    println!("SQL script read successfully! ");

    for statement in init_script.split(";") {
        if !statement.trim().is_empty() {
            println!("Executing SQL statement: {}", statement);
            query(statement).execute(&pool).await?;
        }
    }

    println!("Database initialized successfully!");
    Ok(pool)
}

async fn run() -> Result<(), Error> {
    let init_table = fs::read_to_string("db/init_db.sql").expect("Failed to read init_db.sql file");

    let pool = SqlitePoolOptions::new()
        .connect("sqlite://db/packages.db")
        .await?;

    sqlx::query(&init_table).execute(&pool).await?;

    println!("Database initialized successfully.");

    Ok(())
}
