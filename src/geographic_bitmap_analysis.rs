use image::{DynamicImage, GenericImageView, Pixel};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

pub struct GeographicBitmapAnalysis {
    image: DynamicImage,
    dimensions: (u32, u32),
    rng: ThreadRng,
}

impl GeographicBitmapAnalysis {
    pub fn new(path: &str) -> Result<Self, image::ImageError> {
        let image = image::open(path)?;
        let dimensions = (image.width(), image.height());
        let rng = rand::thread_rng();
        Ok(Self { image, dimensions, rng })
    }

    pub fn get_random_coordinate_on_land(
        &mut self,
        coordinates_record: &str,
    ) -> Result<(f64, f64), io::Error> {
        let saved_coordinates = Self::load_coordinates_record(coordinates_record)?;
        let mut new_coordinates = HashSet::new();
        // generate random x and y position in bitmap (pixels)
        let pixel_x = self.rng.gen_range(0..=self.dimensions.0);
        let pixel_y = self.rng.gen_range(0..=self.dimensions.1);
        let mut latitude: f64 = 0.0;
        let mut longitude: f64 = 0.0;
        let coordinate = format!("{},{}", latitude, longitude);
        // evaluate if position is on land
        if self.is_pixel_black(pixel_x, pixel_y) && !saved_coordinates.contains(&coordinate) {
            new_coordinates.insert(coordinate.clone());
            Self::append_unique_coordinates(coordinates_record, &new_coordinates)?;
            latitude = ((pixel_y as f64 / (self.dimensions.1 as f64 / 180.0)) - 90.0) / -1.0;
            longitude = (pixel_x as f64 / (self.dimensions.0 as f64 / 360.0)) - 180.0;
            return Ok((longitude, latitude));
        } else {
            return self.get_random_coordinate_on_land(coordinates_record);
        }
    }

    fn load_coordinates_record(filename: &str) -> io::Result<HashSet<String>> {
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

    fn is_pixel_black(&self, x: u32, y: u32) -> bool {
        let pixel_color = self.image.get_pixel(x, y);
        let channels = pixel_color.channels();
        let (r, g, b) = (channels[0], channels[1], channels[2]);
        r == 0 && g == 0 && b == 0
    }
}
