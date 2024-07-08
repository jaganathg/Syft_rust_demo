use sqlx::{query, SqlitePool};
use sqlx::{sqlite::SqlitePoolOptions, Error};
use std::{fs, process};
use std::path::Path;
use tokio;
use clap::{Arg, Command};


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

    let flager = Command::new("init_db")
        .version("1.0")
        .about("Initializes the database")
        .arg(Arg::new("reset")
            .short('r')
            .long("reset")
            .help("Recreate all tables even if they exist")
            .num_args(0))
        .get_matches();

    let reset_flag = flager.get_one::<bool>("reset").copied().unwrap_or(false);

    let init_table = fs::read_to_string("db/init_db.sql").expect("Failed to read init_db.sql file");

    let pool = SqlitePoolOptions::new()
        .connect("sqlite://db/packages.db")
        .await?;

    if reset_flag {
        println!("Reinitializing all tables...");
        if let Err(err) = reinitialize_tables(&pool, &init_table).await {
            eprintln!("Error reinitializing tables: {}", err);
            process::exit(1);
        }
    } else {
        println!("New tables initialized...");
        if let Err(err) = sqlx::query(&init_table).execute(&pool).await {
            eprintln!("Error executing SQL script: {}", err);
            process::exit(1);
        }
    }

    // sqlx::query(&init_table).execute(&pool).await?;

    println!("Database initialized successfully.");

    Ok(())
}

async fn reinitialize_tables(pool: &sqlx::SqlitePool, init_table: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Extract table names from the SQL script
    let table_names = extract_table_names(init_table);
    println!("Tables to reinitialize: {:?}", table_names);

    // Drop each table if it exists
    for table_name in table_names {
        let drop_query = format!("DROP TABLE IF EXISTS {};", table_name);
        sqlx::query(&drop_query).execute(pool).await?;
    }

    // Execute the original init script to recreate the tables
    sqlx::query(init_table).execute(pool).await?;

    Ok(())
}

fn extract_table_names(sql_script: &str) -> Vec<String> {
    let mut table_names = Vec::new();

    // Find all table names in the CREATE TABLE statements
    for line in sql_script.lines() {
        if line.trim_start().starts_with("CREATE TABLE") {
            if let Some(table_name) = line.split_whitespace().nth(5) {
                table_names.push(table_name.trim().to_string());
            }
        }
    }

    table_names
}