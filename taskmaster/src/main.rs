mod random_number_generator;

use random_number_generator::RandomNumberGenerator;
use std::io::{self, Write};

fn main() {
    println!("Taskmaster v1.0");

    loop {
        println!("\nAvailable Tasks");
        println!("1. Generate a random number");
        println!("2. Exit");

        print!("\nPlease select a task: ");
        io::stdout().flush().unwrap();

        let mut selection = String::new();
        io::stdin()
            .read_line(&mut selection)
            .expect("Failed to read input");

        match selection.trim() {
            "1" => {
                let generator = RandomNumberGenerator::new();
                generator.generate_random_number();
            }
            "2" => {
                println!("Exiting program. Goodbye!");
                break;
            }
            _ => println!("Invalid selection. Please try again."),
        }
    }
}
