mod fetch_reddit_saved_data;
mod generate_random_number;

use fetch_reddit_saved_data::FetchRedditSavedData;
use generate_random_number::GenerateRandomNumber;
use std::error::Error;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("Taskmaster v1.0");

    loop {
        println!("\nAvailable Tasks");
        println!("1. Generate random number");
        println!("2. Fetch Reddit saved data");

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
                let fetcher: FetchRedditSavedData = FetchRedditSavedData::from_env()?;
                fetcher.fetch_and_save("reddit_saved_items.json").await?;
            }
            _ => {
                println!("Invalid selection. Please try again.");
            }
        }
    }
}
