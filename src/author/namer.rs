use regex::Regex;
use std::error::Error;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use walkdir::WalkDir;

trait FileParser {
    fn extract_author(&self, file_path: &Path) -> Option<String>;
}

struct GenericFileParser {
    re:Regex,
}

impl GenericFileParser {
    fn new() -> Self {
        Self {
            re: Regex::new(r"Copyright\s+\(C\)\s+\d{4}(?:-\d{4})?(?:,\s*\d{4})*(?:,\s*\d{4})?\s+(.*)")
            .unwrap(),
        }
    }
} 

impl FileParser for GenericFileParser {
    fn extract_author(&self, file_path: &Path) -> Option<String> {
        if let Ok(file) = fs::File::open(file_path) {
            for line in io::BufReader::new(file).lines() {
                if let Ok(line) = line {
                    if let Some(caps) = self.re.captures(&line) {
                        return Some(caps[1].to_string());
                    }
                }
            }
        }
        None
    }
}

fn get_parser() -> Box<dyn FileParser> {
    Box::new(GenericFileParser::new())
}

fn process_files_in_directory(directory: &Path) -> Vec<(String, String)> {
    let parser = get_parser();
    let mut results = Vec::new();

    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(extension) = path.extension() {
                if extension == "c" || extension == "h" {
                    if let Some(author) = parser.extract_author(path) {
                        let file_name = path.file_name().unwrap().to_str().unwrap().to_string();
                        results.push((file_name, author));
                    }
                }
            }
        }
    }
    results
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
