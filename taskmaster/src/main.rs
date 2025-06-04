mod check_md_duplicates;
mod convert_image;
mod extract_reddit_saved_data;
mod generate_random_number;

use check_md_duplicates::CheckMdDuplicates;
use convert_image::ConvertImage;
use extract_reddit_saved_data::ExtractRedditSavedData;
use generate_random_number::GenerateRandomNumber;
use std::error::Error;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Taskmaster 0.4.1");

    loop {
        println!("\nAvailable Tasks");
        println!("1. Generate random number");
        println!("2. Convert images from directory");
        println!("3. Extract Reddit saved data");
        println!("4. Check for duplicate lines in .md file");

        print!("\nPlease select a task: ");
        io::stdout().flush().unwrap();

        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read input");

        match selection.trim() {
            "1" => {
                let generator = GenerateRandomNumber::new();
                generator.generate_random_number();
            }
            "2" => {
                let image_converter = ConvertImage::new();
                image_converter.run();
            }
            "3" => {
                let reddit_saved_data_extractor: ExtractRedditSavedData =
                    ExtractRedditSavedData::from_env()?;
                reddit_saved_data_extractor
                    .extract("reddit_saved_items.json")
                    .await?;
            }
            "4" => {
                let duplicate_checker = CheckMdDuplicates::new();
                if let Err(e) = duplicate_checker.run() {
                    println!("Error checking duplicates: {}", e);
                }
            }
            _ => {
                println!("Invalid selection. Please try again.");
            }
        }
    }
}
