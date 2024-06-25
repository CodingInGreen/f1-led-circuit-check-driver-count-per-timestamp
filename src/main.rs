use csv::ReaderBuilder;
use csv::WriterBuilder;
use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;

#[derive(Debug, Deserialize)]
struct Record {
    timestamp: String,
    led_num: u32,
    driver_number: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Open the CSV file
    let file_path = "/Users/hott/eng/f1-led-circuit-check-driver-count-per-timestamp/output_track_data_short_sample_consolidated_timestamps.csv"; // Update this to your file path
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Create a writer for the output CSV
    let output_file = File::create("output.csv")?;
    let mut wtr = WriterBuilder::new().from_writer(output_file);

    // Write the header row
    wtr.write_record(&["timestamp", "driver_count", "duplicates"])?;

    // Initialize a HashMap to store counts and a HashMap to check duplicates
    let mut driver_map: HashMap<String, HashSet<u32>> = HashMap::new();
    let mut duplicates_map: HashMap<String, bool> = HashMap::new();

    // Iterate through records
    for result in rdr.deserialize() {
        let record: Record = result?;
        let timestamp = record.timestamp.clone();

        // Count the drivers and check for duplicates
        let drivers = driver_map.entry(timestamp.clone()).or_insert_with(HashSet::new);
        if !drivers.insert(record.driver_number) {
            duplicates_map.insert(timestamp.clone(), true); // Mark as duplicate if already present
        }
    }

    // Write the results to the output CSV
    for (timestamp, drivers) in driver_map {
        let driver_count = drivers.len();
        let duplicates = duplicates_map.get(&timestamp).unwrap_or(&false);
        wtr.write_record(&[
            timestamp,
            driver_count.to_string(),
            duplicates.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
