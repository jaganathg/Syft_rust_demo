#[allow(unused_imports)]
use polars::prelude::*;
#[allow(unused_imports)]
use serde::Deserialize;
#[allow(unused_imports)]
use serde_json::Value;
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Row, SqlitePool};
use sqlx::{TypeInfo, ValueRef};
use std::error::Error;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::BufReader;
#[allow(unused_imports)]
use tokio::fs::File as TokioFile;
#[allow(unused_imports)]
use tokio::io::AsyncReadExt;

use crate::bd_reader::*;
use crate::sy_reader::*;

mod bd_reader;
mod sy_reader;

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file_path_clean = "sample/sbom_2024-04-29_074207.cdx.cleaned.json";
    let file_path_complete = "sample/sbom_2024-04-29_074207.cdx.json";
    let file_path_syft = "sample/0031_syfts.json";

    let clean_sbom_df = read_sbom_bd_dataframe(&file_path_clean).await?;
    let complete_sbom_df = read_sbom_bd_dataframe(&file_path_complete).await?;
    // let syft_df = read_json_dataframe(&file_path_syft).await?;

    let clean_vuln_df = read_vulner_bd_dataframe(&file_path_clean).await?;
    let complete_vuln_df = read_vulner_bd_dataframe(&file_path_complete).await?;

    let syft_sbom_df = read_sbom_sy_dataframe(&file_path_syft).await?;

    println!(" BD clean sbom");
    println!("{:?}", clean_sbom_df);
    println!(" BD complete sbom");
    println!("{:?}", complete_sbom_df);
    println!(" SY complete sbom");
    println!("{:?}", syft_sbom_df);

    let joined_df = complete_sbom_df.left_join(&clean_sbom_df, ["name"], ["name"])?;

    let non_matching_df = joined_df
        .filter(&joined_df.column("type_right")?.is_null())?
        .select(&["name", "version", "type", "licenses", "supplier", "purl"])?;

    // let joined_df = complete_df.join(&clean_df, &["name"], &["name"], JoinArgs {how: JoinType::Full, ..Default::default() })?;

    // let non_matching_df = joined_df.filter(
    //     &joined_df.column("type_right")?.is_null() ,
    // )?;
    // method 1
    let sy_bd_clean_df = clean_sbom_df.join(&syft_sbom_df, ["name", "version", "type"], ["name", "version", "type"], JoinType::Inner.into())?; 
    let sy_bd_complete_df = complete_sbom_df.join(&syft_sbom_df, ["name", "version", "type"], ["name", "version", "type"], JoinType::Inner.into())?; 

    // method 2
    let sy_bd_clean_2_df = clean_sbom_df
        .clone()
        .lazy()
        .join(
            syft_sbom_df.clone().lazy(), 
            [col("name"), col("version"), col("type")], 
            [col("name"), col("version"), col("type")], 
            JoinArgs::new(JoinType::Inner),
        ).collect()?;  

        let sy_bd_complete_2_df = complete_sbom_df
        .clone()
        .lazy()
        .join(
            syft_sbom_df.clone().lazy(), 
            [col("name"), col("version"), col("type")], 
            [col("name"), col("version"), col("type")], 
            JoinArgs::new(JoinType::Inner),
        ).collect()?;  

    println!("------------------------------------------------");

    println!("{:?}", sy_bd_clean_df);
    println!("{:?}", sy_bd_complete_df);

    println!("{:?}", sy_bd_clean_2_df);
    println!("{:?}", sy_bd_complete_2_df);
    println!("{:?}", clean_vuln_df);
    println!("{:?}", complete_vuln_df);
    // println!("{:?}", syft_df);
    // println!("{:?}", joined_df.get_column_names());
    // println!("{:?}", joined_df);
    // println!("{:?}", non_matching_df);
    println!("------------------------------------------------");

    // let file_path = "blacklist_blackduck.csv";

    // let file = File::create(file_path)?;

    // let mut non_match_df = non_matching_df.clone();

    // CsvWriter::new(file).include_header(true).finish(&mut non_match_df)?;



    Ok(())
}


// ! Not generic function, Not used anymore
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


pub async fn read_table_dataframe(
    pool: &SqlitePool,
    table_name: &str,
) -> Result<DataFrame, Box<dyn Error>> {
    let query = format!("SELECT * FROM {}", table_name);

    // Fetch the rows
    let rows: Vec<SqliteRow> = sqlx::query(&query).fetch_all(pool).await?;

    // Collect column names
    let column_names: Vec<String> = rows
        .first()
        .map(|row| {
            row.columns()
                .iter()
                .map(|col| col.name().to_string())
                .collect()
        })
        .unwrap_or_default();

    // Initialize columns to store data
    let mut columns: Vec<Series> = Vec::new();

    for column_name in &column_names {
        // Infer column type and create a corresponding series
        let first_value = rows
            .first()
            .and_then(|row| row.try_get_raw(column_name.as_str()).ok());

        let type_name = first_value.map(|val| val.type_info().name().to_string());

        match type_name.as_deref() {
            Some("INTEGER") => {
                let col: Vec<Option<i64>> = rows
                    .iter()
                    .map(|row| row.try_get(column_name.as_str()).ok())
                    .collect();
                columns.push(Series::new(column_name, col));
            }
            Some("TEXT") => {
                let col: Vec<Option<String>> = rows
                    .iter()
                    .map(|row| row.try_get(column_name.as_str()).ok())
                    .collect();
                columns.push(Series::new(column_name, col));
            }
            _ => {
                // Handle other types as needed
                let col: Vec<Option<String>> = rows
                    .iter()
                    .map(|row| row.try_get(column_name.as_str()).ok())
                    .collect();
                columns.push(Series::new(column_name, col));
            }
        }
    }

    // Create DataFrame from columns
    let df = DataFrame::new(columns)?;

    Ok(df)
}
