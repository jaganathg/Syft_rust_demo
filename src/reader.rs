use polars::prelude::*;
use sqlx::sqlite::SqlitePool;
use sqlx::Row;

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
    let rows = sqlx::query_as::<_, (String, String)>(&query)
        .fetch_all(pool)
        .await?;

    let names: Vec<&str> = rows.iter().map(|(name, _)| name.as_str()).collect();
    let versions: Vec<&str> = rows.iter().map(|(_, version)| version.as_str()).collect();

    let df = DataFrame::new(vec![
        Series::new("package", names),
        Series::new("version", versions),
    ])
    .expect("Failed to create DataFrame");

    println!("{:?}", df);
    Ok(())
}
