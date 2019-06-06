extern crate progress_streams;

use progress_streams::ProgressWriter;
use std::io::{Write, sink};

fn main() {
    let mut total = 0;
    let mut file = sink();
    let mut writer = ProgressWriter::new(&mut file, |progress: usize| {
        total += progress;
        println!("Written {} Kib", total / 1024);
    });


    let buffer = [0u8; 8192];
    for _ in 0..100_000 {
        writer.write(&buffer).unwrap();
    }
}