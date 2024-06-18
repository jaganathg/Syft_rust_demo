use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sbom {
    // list of artifacts
    artifacts: Vec<Package>,
}


fn main() {
    // Try to read the sbom.json file and return a result.csv file with whitelist and blacklisted artifacts

    let whitelist = vec!["serde", "reqwest"];
    let blacklist = vec!["tokio"];

    let sbom_data = fs::read_to_string("sbom.json")
        .expect("failed to read sbom.json");
    let sbom: Sbom = serde_json::from_str(&sbom_data)
        .expect("failed to parse sbom.json");

    let mut results = Vec::new();

    for package in sbom.artifacts {
        if whitelist.contains(&package.name.as_str()) {
            results.push(format!("{} is whitelisted", package.name));
        } else if blacklist.contains(&package.name.as_str()) {
            results.push(format!("{} is blacklisted", package.name));
        } else {
            results.push(format!("{} is not in whitelist or blacklist", package.name));
        }
    }

    let mut file = File::create("results.csv")
        .expect("failed to create file");

    for result in results {
        writeln!(file, "{}", result)
            .expect("failed to write to file");
    }
}
