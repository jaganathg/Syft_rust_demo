#[allow(non_snake_case)]
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
    #[serde(rename = "bom-ref")]
    bom_ref: Option<String>,
    #[serde(rename = "type")]
    component_type: Option<String>,
    name: Option<String>,
    version: Option<String>,
    cpe: Option<String>,
    purl: Option<String>,
    #[serde(rename = "externalReferences")]
    external_references: Option<Vec<ExternalReference>>,
    properties: Option<Vec<Property>>,
}

#[derive(Debug, Deserialize)]
pub struct ExternalReference {
    url: Option<String>,
    #[serde(rename = "type")]
    ref_type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Property {
    name: Option<String>,
    value: Option<String>,
}

pub async fn read_sbom_sy_dataframe(file_path: &str) -> Result<DataFrame, Box<dyn Error>> {
    // Read the JSON file asynchronously
    let mut file = TokioFile::open(file_path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;

    let v: Value = serde_json::from_str(&contents)?;

    // Extract components array
    let components = v["components"].as_array().unwrap();

    let mut bom_refs = Vec::new();
    let mut types = Vec::new();
    let mut names = Vec::new();
    let mut versions = Vec::new();
    let mut cpes = Vec::new();
    let mut purls = Vec::new();
    let mut external_references_url = Vec::new();
    let mut external_references_type = Vec::new();
    let mut properties_name = Vec::new();
    let mut properties_value = Vec::new();

    for component in components {
        let component: Component = serde_json::from_value(component.clone())?;
        bom_refs.push(component.bom_ref.unwrap_or_default());
        types.push(component.component_type.unwrap_or_default());
        names.push(component.name.unwrap_or_default());
        versions.push(component.version.unwrap_or_default());
        cpes.push(component.cpe.unwrap_or_default());
        purls.push(component.purl.unwrap_or_default());

        let ext_ref_url_str = component
            .external_references
            .as_ref()
            .map_or("None".to_string(), |extreu| {
                extreu.iter()
                    .map(|extr| extr.url.clone().unwrap_or_else(|| "None".to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            });
        external_references_url.push(ext_ref_url_str);

        let ext_ref_type_str = component
            .external_references
            .as_ref()
            .map_or("None".to_string(), |extret| {
                extret.iter()
                    .map(|extr| extr.ref_type.clone().unwrap_or_else(|| "None".to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            });
        external_references_type.push(ext_ref_type_str);

        let prop_name_str = component
            .properties
            .as_ref()
            .map_or("None".to_string(), |propsn| {
                propsn.iter()
                    .map(|prop| prop.name.clone().unwrap_or_else(|| "None".to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            });
        properties_name.push(prop_name_str);

        let prop_value_str = component
            .properties
            .as_ref()
            .map_or("None".to_string(), |propsv| {
                propsv.iter()
                    .map(|prop| prop.value.clone().unwrap_or_else(|| "None".to_string()))
                    .collect::<Vec<String>>()
                    .join(", ")
            });
        properties_value.push(prop_value_str);
    }

    // Create DataFrame
    let df = df![
        "name" => names,
        "version" => versions,
        "type" => types,
        "cpe" => cpes,
        "purl" => purls,
        "ext ref url" => external_references_url,
        "ext ref type" => external_references_type,
    ]?;

    Ok(df)
}