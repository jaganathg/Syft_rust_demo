use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::fs;
use std::path::Path;

use database::action::{add_to_table, check_duplicates};
use database::init::initialize_db;
use scanner::scan::{run_grype_valner, run_syft_scan};
// use reader::readf::read_table_dataframe;

mod database;
mod scanner;
mod cyclonedx_exporter;
// mod reader;

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sbom {
    artifacts: Vec<Package>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Component {
    #[serde(rename = "bom-ref")]
    bom_ref: String,
    name: String,
    version: String,
    // other fields as necessary
}

#[derive(Deserialize, Serialize, Debug)]
struct CycloneDxSbom {
    components: Vec<Component>,
}

#[tokio::main]
async fn main() {
    let target = "/Users/jaganath.gajendran/Documents/J_Study/GitHub/Rust/Diesel_Sqlite_Demo";
    let inter_output = "sbom.json";
    let syft_output = "syft_final.json";
    let grype_output = "grype_final.json";

    run_syft_scan(target, inter_output, &syft_output);
    run_grype_valner(&syft_output, &grype_output);

    let db_url = "db/packages.db";
    let mut pool: Option<SqlitePool> = None;

    // create database if not exist
    if !Path::new(db_url).exists() {
        println!("Database not found, initializing...");
        initialize_db(db_url)
            .await
            .expect("Failed to initialize db");
    } else {
        println!("Database found, connecting...");
        pool = Some(
            SqlitePool::connect(&db_url)
                .await
                .expect("Failed to connect to database"),
        );
    }
    // connect to database
    // let pool = init::initialize_db(db_url)
    //     .await
    //     .expect("Failed to initialize db");

    // Add package and version to whitelist, ensuring no duplicates
    // let package_name = "serde";
    // let package_version = "1.0.203";
    // let exists = init::check_duplicates(&pool, package_name, package_version, "whitelist")
    //     .await
    //     .expect("Failed to check duplicates in whitelist");
    // if !exists {
    //     init::add_to_table(&pool, package_name, package_version, "whitelist")
    //         .await
    //         .expect("Failed to add to whitelist");
    // }

    // Add package and version to blacklist, ensuring no duplicates
    // let package_name = "tokio";
    // let package_version = "1.37.0";
    // let exists = init::check_duplicates(&pool, package_name, package_version, "blacklist")
    //     .await
    //     .expect("Failed to check duplicates in blacklist");
    // if !exists {
    //     init::add_to_table(&pool, package_name, package_version, "blacklist")
    //         .await
    //         .expect("Failed to add to blacklist");
    // }

    let pool = pool.unwrap();

    // Read and parse final.json
    let data = fs::read_to_string("syft_final.json").expect("Failed to read final.json");
    let sbom: CycloneDxSbom = serde_json::from_str(&data).expect("Failed to parse final.json");

    for component in sbom.components {
        let exists = check_duplicates(&pool, &component.name, &component.version, "current")
            .await
            .expect("Failed to check duplicates");
        if !exists {
            add_to_table(&pool, &component.name, &component.version, "current")
                .await
                .expect("Failed to add to current");
        }
    }

    println!("Database initialized and data added successfully");

    // read database and print the tables
    // reader::read_database(&pool)
    //     .await
    //     .expect("Failed to read database");

    // let current_df = read_table_dataframe(&pool, "current")
    //     .await
    //     .expect("Failed to read database using Polars");
    // println!("{:?}", current_df);

    // let component_df = read_table_dataframe(&pool, "component")
    //     .await
    //     .expect("Failed to read database using Polars");
    // println!("{:?}", component_df);

    // drop table
    // database::drop_table(&pool, "whitelist")
    //     .await
    //     .expect("Failed to drop table");
    // database::drop_table(&pool, "blacklist")
    //     .await
    //     .expect("Failed to drop table");
    // database::drop_table(&pool, "current")
    //     .await
    //     .expect("Failed to drop table");
}
