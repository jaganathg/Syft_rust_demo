
use polars::prelude::*;
use serde::Deserialize;
use serde_json::Value;
#[allow(unused_imports)]
use sqlx::sqlite::SqliteRow;
#[allow(unused_imports)]
use sqlx::{Column, Row, SqlitePool};
#[allow(unused_imports)]
use sqlx::{TypeInfo, ValueRef};
use std::error::Error;
#[allow(unused_imports)]
use std::fs::File;
#[allow(unused_imports)]
use std::io::BufReader;
use tokio::fs::File as TokioFile;
use tokio::io::AsyncReadExt;



#[derive(Debug, Deserialize)]
pub struct Component {
    name: String,
    version: String,
    #[serde(rename = "type")]
    component_type: String,
    licenses: Option<Vec<License>>,
    supplier: Option<Supplier>,
    purl: Option<String>,
    pedigree: Option<Pedigree>,
}

#[derive(Debug, Deserialize)]
pub struct License {
    license: LicenseInfo,
}

#[derive(Debug, Deserialize)]
pub struct LicenseInfo {
    id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Supplier {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Pedigree {
    notes: Option<String>,
}


pub async fn read_sbom_bd_dataframe(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
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
    let mut purl = Vec::new();
    let mut pedigree = Vec::new();


    for component in components {
        let component: Component = serde_json::from_value(component.clone())?;
        names.push(component.name.clone());
        versions.push(component.version.clone());
        types.push(component.component_type.clone());
        purl.push(component.purl.clone());

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

        // Handle pedigree
        let pedigree_str = component
            .pedigree
            .as_ref()
            .map_or("None".to_string(), |ped| {
                ped.notes.clone().unwrap_or_else(|| "None".to_string())
            });
        pedigree.push(pedigree_str);

        
    }
    

    // Create DataFrame
    let df = df![
        "name" => names,
        "version" => versions,
        "type" => types,
        "licenses" => licenses,
        "supplier" => suppliers,
        "purl" => purl,
        "pedigree" => pedigree,
    ]?;

    Ok(df)
}


#[derive(Debug, Deserialize)]
pub struct Vulnerability {
    id: String,
    ratings: Option<Vec<Ratings>>,
    affects: Option<Vec<Affect>>, 
}

#[derive(Debug, Deserialize)]
pub struct Ratings {
    source: Source,
    score: Option<f64>, 
    severity: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Source {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Affect {
    #[serde(rename = "ref")]
    ref_value: Option<String>,
}

pub async fn read_vulner_bd_dataframe(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    // Read the JSON file asynchronously
    let mut file = TokioFile::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let v: Value = serde_json::from_str(&contents)?;

    let vulnerabilities = v["vulnerabilities"].as_array().unwrap();

    // Initialize vectors for DataFrame columns
    let mut ids = Vec::new();
    let mut sources = Vec::new();
    let mut scores = Vec::new();
    let mut severities = Vec::new();
    let mut refs = Vec::new();

    // println!("vulnerabilities: {}", vulnerabilities.len());

    for vulner in vulnerabilities {
        let vulner: Vulnerability = serde_json::from_value(vulner.clone())?;
        ids.push(vulner.id.clone());

        let score_str = vulner
            .ratings
            .as_ref()
            .map_or("None".to_string(), |rates| {
                rates.iter()
                    .map(|rate| rate.score.map_or("None".to_string(), |s| s.to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            });
        scores.push(score_str);

        let severity_str = vulner
            .ratings
            .as_ref()
            .map_or("None".to_string(), |rates| {
                rates.iter()
                    .map(|rate| rate.severity.clone().unwrap_or_else(|| "None".to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            });
        severities.push(severity_str);

        let source_str = vulner
            .ratings
            .as_ref()
            .map_or("None".to_string(), |rates| {
                rates.iter()
                    .map(|rate| rate.source.name.clone().unwrap_or_else(|| "None".to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            });
        sources.push(source_str);

        let ref_str = vulner
            .affects
            .as_ref()
            .map_or("None".to_string(), |affects| {
                affects.iter()
                    .map(|affect| affect.ref_value.clone().unwrap_or_else(|| "None".to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            });
        refs.push(ref_str);
    }

    // Create DataFrame
    let df = df![
        "id" => ids,
        "source" => sources,
        "score" => scores,
        "severity" => severities,
        "refs" => refs,
    ]?;

    Ok(df)
}
