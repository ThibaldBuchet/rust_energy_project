use std::fs::File;
use std::io::prelude::*;
use serde_derive::Deserialize;
use csv::Error;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("ogd104_stromproduktion_swissgrid.csv")?;
    
    let mut reader = csv::Reader::from_reader(file);

    #[derive(Debug, Deserialize)]
    struct Energy {
        #[serde(rename = "Datum", default)]  
        date: String,
        #[serde(rename = "Energietraeger", default)]
        energy_type: String,
        #[serde(rename = "Produktion_GWh")]
        production: f64,
    }

    
    let mut all_energy: Vec<Energy> = vec![];

    // Reading and deserializing CSV records
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
    let total_production: f64 = all_energy.iter().map(|energy| energy.production).sum();
    println!("Total energy production: {} GWh", total_production);

    // Calculate production by types of energy production
    let mut production_by_type: HashMap<String, f64> = HashMap::new();
    for energy in all_energy.iter() {
        let counter = production_by_type.entry(energy.energy_type.clone()).or_insert(0.0);
        *counter += energy.production;
    }

    for (energy_type, production) in &production_by_type {
        println!("{}: {} GWh", energy_type, production);
    }

    // Calculate production by years
    let mut production_by_years: HashMap<String, f64> = HashMap::new();
    for energy in all_energy.iter() {
    
        let year = energy.date.split('-').next().unwrap_or("").to_string();
        let years = production_by_years.entry(year).or_insert(0.0);
        *years += energy.production;
    }

    for (year, production) in &production_by_years {
        println!("{}: {} GWh", year, production);
    }

    Ok(())
}
