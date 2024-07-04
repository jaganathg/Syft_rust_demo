use sqlx::{query, SqlitePool};
#[allow(unused_imports)]
use std::fs;
#[allow(unused_imports)]
use std::path::Path;

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

#[allow(dead_code)]
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
