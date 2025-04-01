use rand::Rng;
use std::io::{self, Write};

pub struct GenerateRandomNumber {}

impl GenerateRandomNumber {
    pub fn new() -> Self {
        GenerateRandomNumber {}
    }

    pub fn generate_random_number(&self) {
        print!("Enter maximum value: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<u32>() {
            Ok(max) => {
                let random_number = rand::rng().random_range(1..=max);
                println!("Random number between 1 and {}: {}", max, random_number);
            }
            Err(_) => println!("Please enter a valid number"),
        }
    }
}
