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
    let total_production: f64 = all_energy
        .iter()
        .map(|energy| energy.production)
        .sum();
    println!("Total energy production: {} GWh", total_production);

    // Calculate production by types of energy production
    let mut production_by_type: HashMap<&String, f64> = HashMap::new();
    for energy in all_energy.iter() {
        let counter = production_by_type.entry(&energy.energy_type).or_insert(0.0);
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

    // Calculate production by months
    let mut production_by_months: HashMap<String, f64> = HashMap::new();
    for energy in all_energy.iter() {
        let month = match energy.date.split('-').collect::<Vec<&str>>().get(1) {
            Some(month) => month.to_string(),
            None => "".to_string(),
        };
        let months = production_by_months.entry(month).or_insert(0.0);
        *months += energy.production;
    }

    for (month, production) in &production_by_months {
        println!("{}: {} GWh", month, production);
    }

    //Type of energy that produced the most all years combined


    let most_productive =  production_by_type.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal));
    
    match most_productive {
        Some((most_productive_energy_type, most_production)) => println!("The type of energy that produced the most across all years combined is {} with {} GWh", most_productive_energy_type, most_production),
        None => print!("Aucunne energie productiove")
    }


    //Year with the most production

    if let Some((most_productive_year, most_production)) = production_by_years.iter().max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal)) {
        println!("The year with the most production is {} with {} GWh", most_productive_year, most_production);
    }

    Ok(())
}
