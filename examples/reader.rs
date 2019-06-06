extern crate progress_streams;

use progress_streams::ProgressReader;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut total = 0;
    let mut file = File::open("/dev/urandom").unwrap();
    let mut reader = ProgressReader::new(&mut file, |progress: usize| {
        total += progress;
        println!("Read {} KiB", total / 1024);
    });

    let mut buffer = [0u8; 8192];
    for _ in 0..10_000 {
        reader.read(&mut buffer).unwrap();
    }
}
