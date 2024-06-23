use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::collections::{HashMap};

use anyhow::Result;

#[derive(Debug)]
struct Station {
    minimum: f64,
    maximum: f64,
    counter: u64,
    sum: f64,
}

fn main() {
    process_1brc(&Path::new("input.csv")).expect("unable to finish the challenge");
}

fn process_1brc(path: &Path) -> Result<()> {
    let mut station_map: HashMap<String, Station> = HashMap::new();

    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);

    for line in reader.lines() {
        let line: String = line?;
        let items: Vec<&str> = line.split(';').collect();

        let station_name = items[0].to_string();
        let temperature = items[1].parse::<f64>()?;

        if !station_map.contains_key(&station_name) {
            station_map.insert(station_name, Station{
                minimum: temperature,
                maximum: temperature,
                counter: 1,
                sum: temperature,
            });
        } else {
            if let Some(stored_station) = station_map.get_mut(&station_name) {
                if temperature < stored_station.minimum {
                    stored_station.minimum = temperature;
                }

                if temperature > stored_station.maximum {
                    stored_station.maximum = temperature;
                }

                stored_station.counter += 1;
                stored_station.sum += temperature;
            }
        }
    }

    let mut station_names = station_map.keys().collect::<Vec<&String>>();
    station_names.sort();

    for station_name in station_names.iter() {
        if let Some(station) = station_map.get(*station_name) {
            println!("{};{};{};{}", station_name, station.minimum, station.sum / station.counter as f64, station.maximum);
        }
    }

    Ok(())
}
