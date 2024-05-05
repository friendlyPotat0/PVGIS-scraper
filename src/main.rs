mod geographic_bitmap_analysis;

// use curl::easy::Easy;
use geographic_bitmap_analysis::GeographicBitmapAnalysis;
use std::{env, io};

// Print a web page onto stdout
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // let mut curl = Easy::new();
    // let n: i32 = request_input(&String::from("Enter number of times to scrape: "));
    // !!
    // Create an instance of GeographicBitmapAnalysis
    let mut analysis = GeographicBitmapAnalysis::new("resources/map.png")
        .expect("Failed to create GeographicBitmapAnalysis");

    // Call the get_random_coordinate_on_land method
    let (x, y) = analysis.get_random_coordinate_on_land();

    // Now you can use x and y as needed
    println!("X coordinate: {}", x);
    println!("Y coordinate: {}", y);
    /* if analysis.is_pixel_black(2006, 1079) {
        println!("Black");
    } else {
        println!("White");
    } */
    // !!
    // for _ in 0..n {
    // let coordinates = gba?.get_random_coordinate_on_land("resources/coordinates_record.csv");
    // println!("{}", )?;
    // }

    /* curl.url("https://re.jrc.ec.europa.eu/api/v5_2/seriescalc?lat=51.034&lon=11.836&browser=0&outputformat=json&optimalangles=1&startyear=2005&endyear=2005").unwrap();
    curl.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    })
    .unwrap();
    curl.perform().unwrap(); */

    // println!("{}", curl.response_code().unwrap());
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
