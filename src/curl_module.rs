use curl::easy::Easy;
use std::fs::File;
use std::io::Write;

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

        self.easy.perform().expect("Failed to perform download");
    }
}
