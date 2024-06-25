use csv::ReaderBuilder;
use csv::WriterBuilder;
use serde::Deserialize;
use std::collections::HashSet;
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
    let file_path = "/Users/hott/eng/f1-led-circuit-check-driver-count-per-timestamp/output_track_data_short_sample_consolidated_timestamps.csv";
    let file = File::open(file_path)?;
    let mut rdr = ReaderBuilder::new().from_reader(file);

    // Create a writer for the output CSV
    let output_file = File::create("output.csv")?;
    let mut wtr = WriterBuilder::new().from_writer(output_file);

    // Write the header row
    wtr.write_record(&["timestamp", "driver_count", "duplicates"])?;

    // Initialize a HashMap to store counts and a HashSet to check duplicates
    let mut driver_map = std::collections::HashMap::new();
    let mut driver_set = HashSet::new();

    // Iterate through records
    for result in rdr.deserialize() {
        let record: Record = result?;
        let timestamp = record.timestamp;

        // Count the drivers
        let counter = driver_map.entry(timestamp.clone()).or_insert(0);
        *counter += 1;

        // Check for duplicates
        let key = (timestamp.clone(), record.driver_number);
        if !driver_set.insert(key) {
            driver_map.insert(timestamp.clone(), -1); // Mark as duplicate if already present
        }
    }

    // Write the results to the output CSV
    for (timestamp, count) in driver_map {
        let duplicates = count == -1;
        wtr.write_record(&[
            timestamp,
            (if duplicates { "true" } else { "false" }).to_string(),
            duplicates.to_string(),
        ])?;
    }

    wtr.flush()?;
    Ok(())
}
