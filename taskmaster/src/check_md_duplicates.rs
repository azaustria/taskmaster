use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub struct CheckMdDuplicates;

impl CheckMdDuplicates {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        print!("Enter the path to the .md file: ");
        io::stdout().flush()?;

        let mut file_path = String::new();
        io::stdin().read_line(&mut file_path)?;
        let file_path = file_path.trim();

        if !file_path.ends_with(".md") {
            return Err("File must have a .md extension".into());
        }

        if !Path::new(file_path).exists() {
            return Err(format!("File '{}' does not exist", file_path).into());
        }

        self.check_duplicates(file_path)?;
        Ok(())
    }

    fn check_duplicates(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
        let content = fs::read_to_string(file_path)?;
        let lines: Vec<&str> = content.lines().collect();

        let mut line_tracker: HashMap<String, Vec<usize>> = HashMap::new();

        for (line_number, line) in lines.iter().enumerate() {
            let trimmed_line = line.trim().to_string();

            if trimmed_line.is_empty() {
                continue;
            }

            line_tracker
                .entry(trimmed_line)
                .or_insert_with(Vec::new)
                .push(line_number + 1);
        }

        let mut found_duplicates = false;

        println!("\n=== Duplicate Detection Results ===");
        println!("File: {}", file_path);
        println!();

        for (line_content, line_numbers) in &line_tracker {
            if line_numbers.len() > 1 {
                found_duplicates = true;
                println!("Duplicate found: \"{}\"", line_content);
                println!("  Appears on lines: {:?}", line_numbers);
                println!("  Total occurrences: {}", line_numbers.len());
                println!();
            }
        }

        if !found_duplicates {
            println!("No duplicate lines found in the file.");
        } else {
            println!(
                "Summary: Found duplicates for {} unique lines",
                line_tracker.iter().filter(|(_, v)| v.len() > 1).count()
            );
        }

        Ok(())
    }
}
