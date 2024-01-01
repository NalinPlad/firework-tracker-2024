use std::{io::{self, Write}, time::Duration, fs::{File,OpenOptions}};
use text_io::read;
use chrono::Utc;



fn main() {
    const FILENAME: &str = "data.csv";

    let mut data_file = OpenOptions::new()
        .append(true)
        .open(FILENAME)
        .expect("cannot open file");

    // create a loop that waits for the user to press enter
    loop {
        print!("\nClick enter to record a reading");
        let _: String = read!("{}\n");

        let data = chrono::Local::now();
        data_file
            .write((Utc::now().timestamp_millis().to_string() + "\n").as_bytes())
            .expect("write failed");

        println!("Reading recorded at {}", data);
    }

}
