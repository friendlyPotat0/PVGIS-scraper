mod geographic_bitmap_analysis;
mod curl_module;

use curl_module::CurlModule;
use geographic_bitmap_analysis::GeographicBitmapAnalysis;
use std::{env, io};

// Print a web page onto stdout
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut geographic_bitmap_analysis = GeographicBitmapAnalysis::new("resources/map.png").expect("Failed to create GeographicBitmapAnalysis");
    let mut curl_module = CurlModule::new();
    let n: i32 = request_num_input("Enter number of times to scrape: ");
    let path = request_string_input("Enter path to store scraped files: ");
    let path = path.strip_suffix('/').unwrap_or(&path);
    for i in 1..=n {
        let (latitude, longitude) = geographic_bitmap_analysis.get_random_coordinate_on_land();
        let url = format!("https://re.jrc.ec.europa.eu/api/v5_2/seriescalc?lat={:.3}&lon={:.3}&browser=0&outputformat=json&optimalangles=1", latitude, longitude);
        let filename = format!("{}/timeseries_{:.3}_{:.3}.json", path, latitude, longitude);
        curl_module.download(&url, &filename, i);
    }
}

fn request_num_input(message: &str) -> i32 {
    let mut input = String::new();
    print!("{}", message);
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().parse().expect("Input must be an integer")
}

fn request_string_input(message: &str) -> String {
    let mut input = String::new();
    print!("{}", message);
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}
