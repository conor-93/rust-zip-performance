use std::{fmt, fs};
use std::io::Write;

use image::io::Reader as ImageReader;
use std::io::Cursor;
use std::path::PathBuf;
use std::time::{Duration, Instant};

struct Timer {
    total_expired: Duration,
    started: Option<Instant>,
}

impl Timer {
    fn new() -> Self {
        Timer {
            total_expired: Duration::new(0, 0),
            started: None,
        }
    }

    fn start(&mut self) {
        self.started = Some(Instant::now());
    }

    fn pause(&mut self) {
        if let Some(start) = self.started {
            let now = Instant::now();
            self.total_expired += now.duration_since(start);
            self.started = None;
        } else {
            panic!("Timer is already paused");
        }
    }

    fn resume_if_paused(&mut self) {
        if self.started.is_none() {
            self.started = Some(Instant::now());
        }
    }

    fn resume(&mut self) {
        if self.started.is_none() {
            self.started = Some(Instant::now());
        } else {
            panic!("Timer is already running");
        }
    }

    fn total(&self) -> Duration {
        if let Some(start) = self.started {
            return self.total_expired + start.elapsed();
        }
        self.total_expired
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let total = self.total();
        write!(f, "{}.{:03} seconds", total.as_secs(), total.subsec_millis())
    }
}

fn main() {
    let total_iterations = 5;

    // Warmup
    let mut fake_timer = Timer::new();
    fake_timer.start();
    for _ in 0..5 {
        test_operation(&mut fake_timer);
    }
    for _ in 0..10 {
        unrelated_operation();
    }

    let mut timer = Timer::new();
    timer.start();

    for _ in 0..total_iterations {
        test_operation(&mut timer);
        timer.pause();
        unrelated_operation();
        timer.resume();
    }

    println!("Total elapsed time: {}ms", timer.total_expired.as_millis());
    println!("Time per iteration: {}ms", timer.total_expired.as_millis() / total_iterations);
}

fn test_operation(timer: &mut Timer) {
    timer.pause();

    let options = zip::write::FileOptions::default();

    let data = load_file_bytes();//.expect("Failed to load image");

    let zip_path = PathBuf::from(".").join("output.zip");
    let zip_file = fs::File::create(&zip_path).expect("File::create failed");

    let mut zip = zip::ZipWriter::new(zip_file);

    timer.resume();
    //timer.pause();

    let mut file_name = format!("output{}.png", 1);
    zip.start_file(file_name, options).expect("start_file failed");
    zip.write_all(&data).expect("write_all failed");

    file_name = format!("output{}.png", 2);
    zip.start_file(file_name, options).expect("start_file failed");
    zip.write_all(&data).expect("write_all failed");

    file_name = format!("output{}.png", 3);
    zip.start_file(file_name, options).expect("start_file failed");
    zip.write_all(&data).expect("write_all failed");

    file_name = format!("output{}.png", 4);
    zip.start_file(file_name, options).expect("start_file failed");
    zip.write_all(&data).expect("write_all failed");

    file_name = format!("output{}.png", 5);
    zip.start_file(file_name, options).expect("start_file failed");
    zip.write_all(&data).expect("write_all failed");

    zip.finish();

    //timer.resume();
}

fn unrelated_operation() {
    // Define the size of the vector
    let size = 1_000_000;  // 1 million pixels

    // Initialize a vector with 1 million pixels
    // Each pixel is a tuple (R, G, B), here initialized to black (0, 0, 0)
    let mut pixels = vec![(0u8, 0u8, 0u8); size];

    // Example of accessing and modifying a pixel
    pixels[500_000] = (255, 0, 0); // setting the middle pixel to red

    // Print some pixel values to verify
    //println!("First pixel: {:?}", pixels[0]);
    //println!("Middle pixel: {:?}", pixels[500_000]);
}

fn load_file_bytes() -> Vec<u8> {
    fs::read("no_metadata_large.png").expect("Reading file failed")
}

fn load_image() -> Result<Vec<u8>, image::ImageError> {
    let img_path = "no_metadata_large.png";

    // Open and decode the image
    let img = ImageReader::open(img_path)?.decode()?;

    // Convert the image to bytes
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageOutputFormat::Png)?;

    // `bytes` now contains the PNG encoded image data
    println!("Loaded {} bytes", bytes.len());

    Ok(bytes)
}
