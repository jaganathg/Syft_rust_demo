// use serde_json::Value;
// use std::error::Error;
// use std::fs;

// async fn merge_json_files(file1_path: &str, file2_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
//     // Read and parse the first JSON file (sbom.json)
//     let file1_contents = fs::read_to_string(file1_path)?;
//     let mut sbom_json: Value = serde_json::from_str(&file1_contents)?;

//     // Read and parse the second JSON file (valner.json)
//     let file2_contents = fs::read_to_string(file2_path)?;
//     let valner_json: Value = serde_json::from_str(&file2_contents)?;

//     // Extract matches from the second JSON
//     if let Some(matches) = valner_json.get("matches") {
//         // Add matches to the first JSON
//         sbom_json["matches"] = matches.clone();
//     } else {
//         println!("No 'matches' found in the second JSON file.");
//     }

//     // Serialize the merged JSON to a string
//     let merged_json = serde_json::to_string_pretty(&sbom_json)?;

//     // Write the merged JSON to the output file
//     fs::write(output_path, merged_json)?;

//     println!("Merged SBOM written to {}", output_path);

//     Ok(())
// }

// #[tokio::main]
// async fn main() {
//     let file1_path = "sample/0031_syfts.json";
//     let file2_path = "sample/0031_valner.json";
//     let output_path = "sample/0031_merged.json";

//     if let Err(err) = merge_json_files(file1_path, file2_path, output_path).await {
//         eprintln!("Error merging JSON files: {}", err);
//     }
// }

use serde_json::Value;
use std::error::Error;
use std::fs;

async fn merge_json_files(file1_path: &str, file2_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
    // Read and parse the first JSON file (sbom.json)
    let file1_contents = fs::read_to_string(file1_path)?;
    let mut sbom_json: Value = serde_json::from_str(&file1_contents)?;

    // Read and parse the second JSON file (valner.json)
    let file2_contents = fs::read_to_string(file2_path)?;
    let valner_json: Value = serde_json::from_str(&file2_contents)?;

    // Extract matches from the second JSON
    if let Some(matches) = valner_json.get("matches") {
        // Add matches to the first JSON
        sbom_json["matches"] = matches.clone();
    } else {
        println!("No 'matches' found in the second JSON file.");
    }

    // Serialize the merged JSON to a string
    let merged_json = serde_json::to_string_pretty(&sbom_json)?;

    // Write the merged JSON to the output file
    fs::write(output_path, merged_json)?;

    println!("Merged SBOM written to {}", output_path);

    Ok(())
}

#[tokio::main]
async fn main() {

    let file1_path = "sample/0031_syfts.json";
    let file2_path = "sample/0031_valner.json";
    let output_path = "sample/0031_merged_2.json";

    if let Err(err) = merge_json_files(file1_path, file2_path, output_path).await {
        eprintln!("Error merging JSON files: {}", err);
    }
}