#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn run_syft_scan(target: &str, syft_output: &str, final_output: &str) {
    // run syft scan on target(project)
    let status = Command::new("syft")
        .args(&[target, "-o", &format!("cyclonedx-json={}", syft_output)])
        .status()
        .expect("Failed to run Syft scan");

    if !status.success() {
        eprintln!("Syft scan failed");
    }

    // formating json output
    let mut file = File::create(final_output).expect("Failed to create final_output file");

    let jq_output = Command::new("jq")
        .args(&["."])
        .arg(syft_output)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to run jq command")
        .wait_with_output()
        .expect("Failed to wait for jq command");

    if !jq_output.status.success() {
        eprintln!("jq command failed");
        return;
    }

    // write the formatted json to the file
    file.write_all(&jq_output.stdout)
        .expect("Failed to write final_output file");
}

pub fn run_grype_valner(target: &str, output: &str) {
    let status = Command::new("grype")
        .args(&[&format!("sbom:{}", target), "-o", "json", "--file", output])
        .status()
        .expect("Failed to run Grype valner");

    if !status.success() {
        eprintln!("Grype valner failed");
    }
}
