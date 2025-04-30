mod convert_image;
mod generate_random_number;

use convert_image::ConvertImage;
use generate_random_number::GenerateRandomNumber;
use std::io::{self, Write};

fn main() {
    println!("Taskmaster v1.0");

    loop {
        println!("\nAvailable Tasks");
        println!("1. Generate random number");
        println!("2. Convert images from directory");

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
            _ => {
                println!("Invalid selection. Please try again.");
            }
        }
    }
}
