use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;

#[derive(Debug, Serialize, Deserialize)]
struct Package {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Sbom {
    artifacts: Vec<Package>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let whitelist = vec!["serde", "reqwest"];
    let blacklist = vec!["tokio"];

    let sbom_data = fs::read_to_string("sbom.json")?;
    let sbom: Sbom = serde_json::from_str(&sbom_data)?;

    let mut file = fs::File::create("results.csv")?;

    for package in sbom.artifacts {
        let status = if whitelist.contains(&package.name.as_str()) {
            "whitelisted"
        } else if blacklist.contains(&package.name.as_str()) {
            "blacklisted"
        } else {
            "not listed"
        };
        writeln!(file, "{} : {} --> {}", package.name, package.version, status)?;
    }

    Ok(())
}