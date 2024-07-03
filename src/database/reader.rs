#[allow(unused_imports)]
use polars::prelude::*;
use sqlx::sqlite::SqliteRow;
use sqlx::{ValueRef, TypeInfo};
use sqlx::{Row, Column, SqlitePool};
use std::error::Error;

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

// pub async fn read_database_polars(pool: &SqlitePool, table_name: &str) -> Result<(), sqlx::Error> {
//     let query = format!("SELECT * FROM {}", table_name);
//     let rows: Vec<(i32, String, String)> = sqlx::query_as(&query)
//         .fetch_all(pool)
//         .await?;

//     let ids: Vec<i32> = rows.iter().map(|(id, _, _)| *id).collect();
//     let names: Vec<&str> = rows.iter().map(|(_, name, _)| name.as_str()).collect();
//     let versions: Vec<&str> = rows.iter().map(|(_, _, version)| version.as_str()).collect();

//     let df = DataFrame::new(vec![
//         Series::new("id", ids),
//         Series::new("name", names),
//         Series::new("version", versions),
//     ]).expect("Failed to create DataFrame");

//     println!("{:?}", df);
//     Ok(())
// }

pub async fn read_dataframe(pool: &SqlitePool, table_name: &str) -> Result<DataFrame, Box<dyn Error>> {

    let query = format!("SELECT * FROM {}", table_name);

    // Fetch the rows
    let rows: Vec<SqliteRow> = sqlx::query(&query)
        .fetch_all(pool)
        .await?;

    // Collect column names
    let column_names: Vec<String> = rows.first().map(|row| {
        row.columns().iter().map(|col| col.name().to_string()).collect()
    }).unwrap_or_default();

    // Initialize columns to store data
    let mut columns: Vec<Series> = Vec::new();

    for column_name in &column_names {
        // Infer column type and create a corresponding series
        let first_value = rows.first().and_then(|row| row.try_get_raw(column_name.as_str()).ok());

        let type_name = first_value.map(|val| val.type_info().name().to_string());

        match type_name.as_deref() {
            Some("INTEGER") => {
                let col: Vec<Option<i64>> = rows.iter().map(|row| row.try_get(column_name.as_str()).ok()).collect();
                columns.push(Series::new(column_name, col));
            }
            Some("TEXT") => {
                let col: Vec<Option<String>> = rows.iter().map(|row| row.try_get(column_name.as_str()).ok()).collect();
                columns.push(Series::new(column_name, col));
            }
            _ => {
                // Handle other types as needed
                let col: Vec<Option<String>> = rows.iter().map(|row| row.try_get(column_name.as_str()).ok()).collect();
                columns.push(Series::new(column_name, col));
            }
        }
    }

    // Create DataFrame from columns
    let df = DataFrame::new(columns)?;

    Ok(df)
  
}
