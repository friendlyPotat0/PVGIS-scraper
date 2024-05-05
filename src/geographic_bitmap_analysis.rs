use image::{DynamicImage, GenericImageView, Pixel};
use rand::rngs::ThreadRng;
use rand::Rng;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, BufWriter, Write};

pub struct GeographicBitmapAnalysis {
    image: DynamicImage,
    dimensions: (u32, u32),
    latitude: f32,
    longitude: f32,
    rng: ThreadRng,
}

impl GeographicBitmapAnalysis {
    pub fn new(path: &str) -> Result<Self, image::ImageError> {
        let image = image::open(path)?;
        let latitude = 0.0;
        let longitude = 0.0;
        let dimensions = (image.width(), image.height());
        let rng = rand::thread_rng();
        Ok(Self {
            image,
            dimensions,
            latitude,
            longitude,
            rng,
        })
    }

    pub fn get_random_coordinate_on_land(&mut self) -> (f32, f32) {
        loop {
            // generate random x and y position in bitmap (pixels)
            let pixel_x = self.rng.gen_range(0..self.dimensions.0);
            let pixel_y = self.rng.gen_range(0..self.dimensions.1);
            // evaluate if position is on land
            if self.is_pixel_black(pixel_x, pixel_y) {
                self.latitude = ((pixel_y as f32 / (self.dimensions.1 as f32 / 180.0)) - 90.0) / -1.0;
                self.longitude = (pixel_x as f32 / (self.dimensions.0 as f32 / 360.0)) - 180.0;
                break;
            }
        }
        return (self.latitude, self.longitude);
    }

    pub fn is_pixel_black(&self, x: u32, y: u32) -> bool {
        let pixel_color = self.image.get_pixel(x, y);
        let channels = pixel_color.channels();
        let (r, g, b) = (channels[0], channels[1], channels[2]);
        r == 0 && g == 0 && b == 0
    }
}
