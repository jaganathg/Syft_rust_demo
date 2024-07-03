#[allow(unused_imports)]
use polars::prelude::*;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;

#[allow(dead_code)]
pub async fn read_database(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Print whitelist table
    let rows = sqlx::query("SELECT * FROM whitelist")
        .fetch_all(pool)
        .await?;
    println!(" Whitelist table:");
    for row in rows {
        let package: String = row.get("package");
        let version: String = row.get("version");
        println!(" Package: {}, Version: {}", package, version);
    }

    // Print blacklist table
    let rows = sqlx::query("SELECT * FROM blacklist")
        .fetch_all(pool)
        .await?;
    println!(" Blacklist table:");
    for row in rows {
        let package: String = row.get("package");
        let version: String = row.get("version");
        println!(" Package: {}, Version: {}", package, version);
    }

    // Print current table
    let rows = sqlx::query("SELECT * FROM current").fetch_all(pool).await?;
    println!(" Current table:");
    for row in rows {
        let package: String = row.get("package");
        let version: String = row.get("version");
        println!(" Package: {}, Version: {}", package, version);
    }

    Ok(())
}

pub async fn read_database_polars(pool: &SqlitePool, table_name: &str) -> Result<(), sqlx::Error> {
    let query = format!("SELECT * FROM {}", table_name);
    let rows: Vec<(i32, String, String)> = sqlx::query_as(&query)
        .fetch_all(pool)
        .await?;

    let ids: Vec<i32> = rows.iter().map(|(id, _, _)| *id).collect();
    let names: Vec<&str> = rows.iter().map(|(_, name, _)| name.as_str()).collect();
    let versions: Vec<&str> = rows.iter().map(|(_, _, version)| version.as_str()).collect();

    let df = DataFrame::new(vec![
        Series::new("id", ids),
        Series::new("name", names),
        Series::new("version", versions),
    ]).expect("Failed to create DataFrame");

    println!("{:?}", df);
    Ok(())
}
