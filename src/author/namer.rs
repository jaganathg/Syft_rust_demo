use regex::Regex;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

trait FileParser {
    fn extract_author(&self, file_path: &Path) -> Option<String>;
}

struct CFileParser;

impl FileParser for CFileParser {
    fn extract_author(&self, file_path: &Path) -> Option<String> {
        let re =
            Regex::new(r"Copyright\s+\(C\)\s+\d{4}(?:-\d{4})?(?:,\s*\d{4})*(?:,\s*\d{4})?\s+(.*)")
                .unwrap();
        extract_author_from_file(file_path, &re)
    }
}

struct HFileParser;

impl FileParser for HFileParser {
    fn extract_author(&self, file_path: &Path) -> Option<String> {
        let re =
            Regex::new(r"Copyright\s+\(C\)\s+\d{4}(?:-\d{4})?(?:,\s*\d{4})*(?:,\s*\d{4})?\s+(.*)")
                .unwrap();
        extract_author_from_file(file_path, &re)
    }
}

fn extract_author_from_file(file_path: &Path, re: &Regex) -> Option<String> {
    if let Ok(file) = fs::File::open(file_path) {
        for line in io::BufReader::new(file).lines() {
            if let Ok(line) = line {
                if let Some(caps) = re.captures(&line) {
                    return Some(caps[1].to_string());
                }
            }
        }
    }
    None
}

fn get_parser(file_extension: &str) -> Box<dyn FileParser> {
    match file_extension {
        "c" => Box::new(CFileParser),
        "h" => Box::new(HFileParser),
        // Add other file types and parsers here
        _ => panic!("Unsupported file type"),
    }
}

fn process_files_in_directory(directory: &Path) -> Vec<(String, String)> {
    if !directory.is_dir() {
        return vec![];
    }

    fs::read_dir(directory)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if let Some(extension_str) = extension.to_str() {
                        let parser = get_parser(extension_str);
                        if let Some(author) = parser.extract_author(&path) {
                            return Some((path.file_name()?.to_str()?.to_string(), author));
                        }
                    }
                }
            }
            None
        })
        .collect()
}

fn write_to_csv(file_name: &str, data: Vec<(String, String)>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::Writer::from_path(file_name)?;
    wtr.write_record(&["File Name", "Author"])?;

    for (file_name, author) in data {
        wtr.write_record(&[file_name, author])?;
    }

    wtr.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let directory_path = Path::new("sample/zlib/src");
    let output_csv = "authors.csv";

    let results = process_files_in_directory(directory_path);
    write_to_csv(output_csv, results)?;

    Ok(())
}
