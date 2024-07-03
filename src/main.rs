#[allow(unused_imports)]
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

use database::{init, reader};
use syfter::scan;

mod database;
mod syfter;

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sbom {
    artifacts: Vec<Package>,
}


#[tokio::main]
async fn main() {
    let target = "/Users/jaganath.gajendran/Documents/J_Study/GitHub/Rust/Diesel_Sqlite_Demo";
    let inter_output = "sbom.json";
    let syft_output = "syft_final.json";
    let grype_output = "grype_final.csv";

    scan::run_syft_scan(target, inter_output, &syft_output);
    scan::run_grype_valner(&syft_output, &grype_output);

    let db_url = "db/packages.db";

    // connect to database
    let pool = init::initialize_db(db_url)
        .await
        .expect("Failed to initialize db");

    // Add package and version to whitelist, ensuring no duplicates
    let package_name = "serde";
    let package_version = "1.0.203";
    let exists = init::check_duplicates(&pool, package_name, package_version, "whitelist")
        .await
        .expect("Failed to check duplicates in whitelist");
    if !exists {
        init::add_to_table(&pool, package_name, package_version, "whitelist")
            .await
            .expect("Failed to add to whitelist");
    }

    // Add package and version to blacklist, ensuring no duplicates
    let package_name = "tokio";
    let package_version = "1.37.0";
    let exists = init::check_duplicates(&pool, package_name, package_version, "blacklist")
        .await
        .expect("Failed to check duplicates in blacklist");
    if !exists {
        init::add_to_table(&pool, package_name, package_version, "blacklist")
            .await
            .expect("Failed to add to blacklist");
    }

    // Read and parse final.json
    let data = fs::read_to_string("syft_final.json").expect("Failed to read final.json");
    let sbom: Sbom = serde_json::from_str(&data).expect("Failed to parse final.json");

    for package in sbom.artifacts {
        let exists = init::check_duplicates(&pool, &package.name, &package.version, "current")
            .await
            .expect("Failed to check duplicates");
        if !exists {
            init::add_to_table(&pool, &package.name, &package.version, "current")
                .await
                .expect("Failed to add to current");
        }
    }

    println!("Database initialized and data added successfully");

    // read database and print the tables
    reader::read_database(&pool)
        .await
        .expect("Failed to read database");

    reader::read_database_polars(&pool, "current")
        .await
        .expect("Failed to read database using Polars");

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
