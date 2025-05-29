use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use webp::Encoder;

pub struct ConvertImage {
    quality: f32,
}

impl ConvertImage {
    pub fn new() -> Self {
        ConvertImage { quality: 95.0 }
    }

    pub fn run(&self) {
        print!("Enter directory: ");
        io::stdout().flush().unwrap();
        let mut directory = String::new();
        io::stdin()
            .read_line(&mut directory)
            .expect("Failed to read input");
        let directory = directory.trim();

        print!("Delete original files after conversion? (y/n) [default: n]: ");
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

        println!("Auto-selecting best quality: Lossless for PNGs, High quality for JPGs");
        match self.convert_directory_recursive(dir_path, delete_originals) {
            Ok(count) => println!("Successfully converted {} images", count),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn convert_directory(&self, dir: &Path, delete_originals: bool) -> Result<usize, String> {
        let mut count = 0;

        for entry in fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    let ext_lower = ext.to_string_lossy().to_lowercase();
                    if ext_lower == "jpg" || ext_lower == "jpeg" || ext_lower == "png" {
                        match self.convert_image(&path) {
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
        delete_originals: bool,
    ) -> Result<usize, String> {
        let mut count = 0;

        count += self.convert_directory(dir, delete_originals)?;

        for entry in fs::read_dir(dir).map_err(|e| format!("Failed to read directory: {}", e))? {
            let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
            let path = entry.path();

            if path.is_dir() {
                count += self.convert_directory_recursive(&path, delete_originals)?;
            }
        }

        Ok(count)
    }

    fn convert_image(&self, image_path: &Path) -> Result<(), String> {
        let img = image::open(image_path).map_err(|e| format!("Failed to open image: {}", e))?;

        let is_png = if let Some(ext) = image_path.extension() {
            let ext_lower = ext.to_string_lossy().to_lowercase();
            ext_lower == "png"
        } else {
            false
        };

        let encoded_webp = if is_png {
            if img.color().has_alpha() {
                let rgba_img = img.to_rgba8();
                let encoder =
                    Encoder::from_rgba(rgba_img.as_raw(), rgba_img.width(), rgba_img.height());
                encoder.encode_lossless()
            } else {
                let rgb_img = img.to_rgb8();
                let encoder =
                    Encoder::from_rgb(rgb_img.as_raw(), rgb_img.width(), rgb_img.height());
                encoder.encode_lossless()
            }
        } else {
            let rgb_img = img.to_rgb8();
            let encoder = Encoder::from_rgb(rgb_img.as_raw(), rgb_img.width(), rgb_img.height());
            encoder.encode(self.quality)
        };

        let mut output_path = PathBuf::from(image_path);
        output_path.set_extension("webp");

        fs::write(&output_path, &*encoded_webp)
            .map_err(|e| format!("Failed to write WebP image: {}", e))?;

        Ok(())
    }
}

impl Default for ConvertImage {
    fn default() -> Self {
        Self::new()
    }
}
