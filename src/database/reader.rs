#[allow(unused_imports)]
use polars::prelude::*;
use serde::Deserialize;
use serde_json::Value;
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Row, SqlitePool};
use sqlx::{TypeInfo, ValueRef};
use std::error::Error;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::BufReader;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;


#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file_path = "sample/sbom_2024-04-29_074207.cdx.cleaned.json";
    let file_path = "sample/sbom_2024-04-29_074207.cdx.json";
    let df = read_json_dataframe(file_path).await?;

    println!("{:?}", df);

    Ok(())
}


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

#[derive(Debug, Deserialize)]
pub struct Component {
    name: String,
    version: String,
    #[serde(rename = "type")]
    component_type: String,
    licenses: Option<Vec<License>>,
    supplier: Option<Supplier>,
    properties: Option<Vec<Property>>,
}

#[derive(Debug, Deserialize)]
pub struct License {
    license: LicenseInfo,
}

#[derive(Debug, Deserialize)]
pub struct LicenseInfo {
    id: Option<String>,
    _name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Supplier {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Property {
    name: String,
    value: String,
}

pub async fn read_json_dataframe(file_path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    // Read the JSON file asynchronously
    let mut file = TokioFile::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let v: Value = serde_json::from_str(&contents)?;

    // Extract components array
    let components = v["components"].as_array().unwrap();

    // Initialize vectors for DataFrame columns
    let mut names = Vec::new();
    let mut versions = Vec::new();
    let mut types = Vec::new();
    let mut licenses = Vec::new();
    let mut suppliers = Vec::new();
    let mut properties = Vec::new();

    for component in components {
        let component: Component = serde_json::from_value(component.clone())?;
        names.push(component.name.clone());
        versions.push(component.version.clone());
        types.push(component.component_type.clone());

        // Handle multiple licenses
        let license_str = component
            .licenses
            .as_ref()
            .map_or("None".to_string(), |lics| {
                lics.iter()
                    .map(|lic| lic.license.id.clone().unwrap_or_else(|| "None".to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            });
        licenses.push(license_str);

        // Handle supplier
        let supplier_str = component
            .supplier
            .as_ref()
            .map_or("None".to_string(), |sup| {
                sup.name.clone().unwrap_or_else(|| "None".to_string())
            });
        suppliers.push(supplier_str);

        // Handle properties
        let properties_str = component
            .properties
            .as_ref()
            .map_or("None".to_string(), |props| {
                props
                    .iter()
                    .map(|prop| format!("{}: {}", prop.name, prop.value))
                    .collect::<Vec<String>>()
                    .join(", ")
            });
        properties.push(properties_str);
    }

    // Create DataFrame
    let df = df![
        "name" => names,
        "version" => versions,
        "type" => types,
        "licenses" => licenses,
        "supplier" => suppliers,
        "properties" => properties,
    ]?;

    Ok(df)
}

