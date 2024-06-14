use std::fs::File;
use std::io::prelude::*;
use serde_derive::Deserialize;
use csv::Error;

fn main() {
    let mut file = File::open("ogd104_stromproduktion_swissgrid.csv").expect("Can't open the file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Can't read the file!");

    #[derive(Debug, Deserialize)]
    struct Energy {
        #[serde(rename = "Datum", default)]  
        date: String,
        #[serde(rename = "Energietraeger", default)]
        energy_type: String,
        #[serde(rename = "Produktion_GWh")]
        production: f64, 
    }

    
    // Reading csv
    let mut reader = csv::Reader::from_reader(contents.as_bytes());
    let mut all_energy: Vec<Energy> = vec![];

    for result in reader.deserialize::<Energy>() {
        match result {
            Ok(record) => {
                all_energy.push(record);
            },
            Err(err) => {
                eprintln!("Error parsing record: {}", err);
            }
        }
    }

    // Compute total energy production
    let mut total_production = 0.0;
    for energy in all_energy.iter() {
        total_production += energy.production;

    }




    println!("Total energy production: {}", total_production);
}
