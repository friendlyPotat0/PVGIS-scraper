use image::{DynamicImage, GenericImageView, Pixel};
use rand::Rng;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

pub struct GeographicBitmapAnalysis {
    image: DynamicImage,
}

impl GeographicBitmapAnalysis {
    pub fn new(path: &str) -> Result<Self, image::ImageError> {
        let image = image::open(path)?;
        Ok(Self { image })
    }

    pub fn get_random_coordinate_on_land() -> io::Result<()> {
        let coordinates_path = "resources/coordinates.txt";
        let existing_coordinates = Self::load_coordinates(coordinates_path)?;
        let mut new_coordinates = HashSet::new();
        let mut rng = rand::thread_rng();
        loop {
            let latitude = rng.gen_range(-90.0..=90.0);
            let longitude = rng.gen_range(-180.0..=180.0);
            let coordinate = format!("{},{}", latitude, longitude);
            // Check if the generated coordinate is already present
            if !existing_coordinates.contains(&coordinate) {
                // If not, insert it to existing_coordinates set
                new_coordinates.insert(coordinate.clone());
            }
            if new_coordinates.len() >= n as usize {
                break;
            }
        }
        // Append new unique coordinates to the file
        Self::append_unique_coordinates(coordinates_path, &new_coordinates)?;
        // Print the generated coordinates
        println!("Generated coordinates:");
        println!("{:?}", new_coordinates);
        Ok(())
    }

    fn load_coordinates(filename: &str) -> io::Result<HashSet<String>> {
        let mut coordinates_set = HashSet::new();
        if let Ok(file) = File::open(filename) {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                if let Ok(coord) = line {
                    coordinates_set.insert(coord);
                }
            }
        }
        Ok(coordinates_set)
    }

    fn append_unique_coordinates(filename: &str, coordinates: &HashSet<String>) -> io::Result<()> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(filename)?;
        let mut writer = BufWriter::new(file);
        for coord in coordinates {
            writeln!(writer, "{}", coord)?;
        }
        writer.flush()?;
        Ok(())
    }

    // Check if a pixel at given coordinates is black
    pub fn is_black_pixel(&self, x: u32, y: u32) -> bool {
        let pixel_color = self.image.get_pixel(x, y);
        let channels = pixel_color.channels();
        let (r, g, b) = (channels[0], channels[1], channels[2]);
        r == 0 && g == 0 && b == 0
    }
}
