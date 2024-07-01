use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;

mod database;
mod reader;
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

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let whitelist = vec!["serde", "reqwest"];
//     let blacklist = vec!["tokio"];

//     let sbom_data = fs::read_to_string("sbom.json")?;
//     let sbom: Sbom = serde_json::from_str(&sbom_data)?;

//     let mut file = fs::File::create("results.csv")?;

//     for package in sbom.artifacts {
//         let status = if whitelist.contains(&package.name.as_str()) {
//             "whitelisted"
//         } else if blacklist.contains(&package.name.as_str()) {
//             "blacklisted"
//         } else {
//             "not listed"
//         };
//         writeln!(file, "{} : {} --> {}", package.name, package.version, status)?;
//     }

//     Ok(())
// }

#[tokio::main]
async fn main() {
    let target = "/Users/jaganath.gajendran/Documents/J_Study/GitHub/Rust/Diesel_Sqlite_Demo";
    let syft_output = "sbom.json";
    let final_output = "final.json";

    syfter::run_syft_scan(target, syft_output, final_output);

    let db_url = "db/packages.db";

    // connect to database
    let pool = database::initialize_db(db_url)
        .await
        .expect("Failed to initialize db");

    // Add package and version to whitelist, ensuring no duplicates
    let package_name = "serde";
    let package_version = "1.0.203";
    let exists = database::check_duplicates(&pool, package_name, package_version, "whitelist")
        .await
        .expect("Failed to check duplicates in whitelist");
    if !exists {
        database::add_to_table(&pool, package_name, package_version, "whitelist")
            .await
            .expect("Failed to add to whitelist");
    }

    // Add package and version to blacklist, ensuring no duplicates
    let package_name = "tokio";
    let package_version = "1.37.0";
    let exists = database::check_duplicates(&pool, package_name, package_version, "blacklist")
        .await
        .expect("Failed to check duplicates in blacklist");
    if !exists {
        database::add_to_table(&pool, package_name, package_version, "blacklist")
            .await
            .expect("Failed to add to blacklist");
    }

    // Read and parse final.json
    let data = fs::read_to_string("final.json").expect("Failed to read final.json");
    let sbom: Sbom = serde_json::from_str(&data).expect("Failed to parse final.json");

    for package in sbom.artifacts {
        let exists = database::check_duplicates(&pool, &package.name, &package.version, "current")
            .await
            .expect("Failed to check duplicates");
        if !exists {
            database::add_to_table(&pool, &package.name, &package.version, "current")
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
