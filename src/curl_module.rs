use curl::easy::Easy;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub struct CurlModule {
    easy: Easy,
}

impl CurlModule {
    pub fn new() -> Self {
        Self { easy: Easy::new() }
    }

    pub fn download(&mut self, url: &String, filename: &String) {
        let mut file = File::create(filename).expect("Failed to create file");

        self.easy.url(url).expect("Failed to set URL");

        self.easy.write_function(move |data| {
            file.write_all(data).expect("Failed to write data to file");
            Ok(data.len())
        }).expect("Failed to set write function");

        let filename = Path::new(filename).file_name().unwrap().to_string_lossy().into_owned();
        self.easy.progress(true).unwrap();
        self.easy.progress_function(move |_dltotal, dlnow, _ultotal, _ulnow| {
            print!("\r{}: {} received bytes ", filename, dlnow);
            std::io::stdout().flush().unwrap();
            true
        }).unwrap();
        println!("");

        self.easy.perform().expect("Failed to perform download");
    }
}
