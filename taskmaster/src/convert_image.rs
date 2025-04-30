use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use webp::Encoder;

pub struct ConvertImage {
    quality: u8,
}

impl ConvertImage {
    pub fn new() -> Self {
        ConvertImage { quality: 80 }
    }

    pub fn run(&self) {
        print!("Enter directory path containing JPG images: ");
        io::stdout().flush().unwrap();
        let mut directory = String::new();
        io::stdin()
            .read_line(&mut directory)
            .expect("Failed to read input");
        let directory = directory.trim();

        let quality = self.quality;

        print!("Delete original JPG files after conversion? (y/n) [default: n]: ");
        io::stdout().flush().unwrap();
        let mut delete_input = String::new();
        io::stdin()
            .read_line(&mut delete_input)
            .expect("Failed to read input");
        let delete_originals = delete_input.trim().to_lowercase() == "y";

        let dir_path = Path::new(directory);
        if !dir_path.exists() || !dir_path.is_dir() {
            println!("Error: '{}' is not a valid directory", directory);
            return;
        }

        match self.convert_directory_recursive(dir_path, quality, delete_originals) {
            Ok(count) => println!("Successfully converted {} images", count),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn convert_directory(
        &self,
        dir: &Path,
        quality: u8,
        delete_originals: bool,
    ) -> Result<usize, String> {
        let mut count = 0;

        for entry in fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext.to_string_lossy().to_lowercase() == "jpg"
                        || ext.to_string_lossy().to_lowercase() == "jpeg"
                    {
                        match self.convert_image(&path, quality) {
                            Ok(_) => {
                                count += 1;
                                println!("Converted: {}", path.display());

                                if delete_originals {
                                    if let Err(e) = fs::remove_file(&path) {
                                        println!(
                                            "Warning: Could not delete original file {}: {}",
                                            path.display(),
                                            e
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Warning: Failed to convert {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }

        Ok(count)
    }

    fn convert_directory_recursive(
        &self,
        dir: &Path,
        quality: u8,
        delete_originals: bool,
    ) -> Result<usize, String> {
        let mut count = 0;

        count += self.convert_directory(dir, quality, delete_originals)?;

        for entry in fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                count += self.convert_directory_recursive(&path, quality, delete_originals)?;
            }
        }

        Ok(count)
    }

    fn convert_image(&self, jpg_path: &Path, quality: u8) -> Result<(), String> {
        let img = image::open(jpg_path).map_err(|e| format!("Failed to open image: {}", e))?;
        let rgb_img = img.to_rgb8();
        let encoder = Encoder::from_rgb(rgb_img.as_raw(), rgb_img.width(), rgb_img.height());
        let encoded_webp = encoder.encode(f32::from(quality));

        let mut output_path = PathBuf::from(jpg_path);
        output_path.set_extension("webp");

        fs::write(&output_path, &*encoded_webp)
            .map_err(|e| format!("Failed to write WebP image: {}", e))?;

        Ok(())
    }
}
