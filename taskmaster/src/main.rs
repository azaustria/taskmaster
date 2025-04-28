mod generate_random_number;

use generate_random_number::GenerateRandomNumber;
use std::io::{self, Write};

fn main() {
    println!("Taskmaster v1.0");

    loop {
        println!("\nAvailable Tasks");
        println!("1. Generate random number");

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
            _ => {
                println!("Invalid selection. Please try again.");
            }
        }
    }
}
