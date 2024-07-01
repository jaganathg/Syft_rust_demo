use sqlx::{query, SqlitePool};
use std::fs;
use std::path::Path;

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

pub async fn add_to_table(
    pool: &SqlitePool,
    package: &str,
    version: &str,
    table_name: &str,
) -> Result<(), sqlx::Error> {
    let qry = format!(
        "INSERT INTO {} (package, version) VALUES (?, ?)",
        table_name
    );
    query(&qry)
        .bind(package)
        .bind(version)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn drop_table(pool: &SqlitePool, table_name: &str) -> Result<(), sqlx::Error> {
    let qry = format!("DROP TABLE IF EXISTS {}", table_name);
    query(&qry).execute(pool).await?;
    println!("Table {} dropped successfully!", table_name);
    Ok(())
}

pub async fn check_duplicates(
    pool: &SqlitePool,
    package: &str,
    version: &str,
    table_name: &str,
) -> Result<bool, sqlx::Error> {
    let qry = format!(
        "SELECT COUNT(*) FROM {} WHERE package = ? AND version = ?",
        table_name
    );
    let result = sqlx::query_scalar::<_, i64>(&qry)
        .bind(package)
        .bind(version)
        .fetch_one(pool)
        .await?;

    Ok(result > 0)
}
