mod geographic_bitmap_analysis;

use curl::easy::Easy;
use geographic_bitmap_analysis::GeographicBitmapAnalysis;
use rand::Rng;
use std::collections::HashSet;
use std::fs::{File, OpenOptions};
use std::io::{self, stdout, BufRead, BufReader, BufWriter, Write};

// Print a web page onto stdout
fn main() -> io::Result<()> {
    // let mut curl = Easy::new();
    let n: i32 = request_input(&String::from("Enter number of times to scrape: "));
    let gba = GeographicBitmapAnalysis::new("resources/map.bpm");
    for _ in 0..n {
        println!("{}", gba?.get_random_coordinate_on_land())?;
    }

    /* curl.url("https://re.jrc.ec.europa.eu/api/v5_2/seriescalc?lat=51.034&lon=11.836&browser=0&outputformat=json&optimalangles=1&startyear=2005&endyear=2005").unwrap();
    curl.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    curl.perform().unwrap(); */

    // println!("{}", curl.response_code().unwrap());
    Ok(())
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
