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
    let n: i32 = request_input(&String::from("Enter number of times to scrape: "));
    for _ in 0..n {
        let (latitude, longitude) = geographic_bitmap_analysis.get_random_coordinate_on_land();
        let url = format!("https://re.jrc.ec.europa.eu/api/v5_2/seriescalc?lat={:.3}&lon={:.3}&browser=0&outputformat=json&optimalangles=1", latitude, longitude);
        let filename = format!("resources/solar-radiation-database/{:.3}_{:.3}.json", latitude, longitude);
        curl_module.download(&url, &filename);
    }
}

fn request_input(message: &String) -> i32 {
    let mut num = String::new();
    print!("{}", message);
    io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line");
    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return -1;
        }
    };
    return num;
}
