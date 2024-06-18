use csv::Error;
use itertools::Itertools;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

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
            }
            Err(err) => {
                eprintln!("Error parsing record: {}", err);
            }
        }
    }

    // Compute total energy production
    let total_production: f64 = all_energy.iter().map(|energy| energy.production).sum();
    println!("Total energy production: {} GWh", total_production);


    println!("-----------------------------------------------------------------------------------------------------");


    // Calculate production by types of energy production
    let production_by_type: HashMap<&String, f64> =
        all_energy.iter().fold(HashMap::new(), |mut map, energy| {
            let counter = map.entry(&energy.energy_type).or_insert(0.0);
            *counter += energy.production;
            map
        });

    production_by_type
        .iter()
        .for_each(|(energy_type, production)| {
            println!("{}: {} GWh", energy_type, production);
        });

    println!("--------------------------------------------------------------------------------------------------");
    all_energy
        .iter()
        .into_grouping_map_by(|element| element.energy_type.clone())
        .fold(0.0, |acc, _key, elements| acc + elements.production)
        .iter()
        .for_each(|total| println!("Total production for {}, {} Gwh", total.0, total.1));

    println!("-----------------------------------------------------------------------------------------------------");


    // Calculate production by years
    let mut production_by_years: HashMap<String, f64> = HashMap::new();

    all_energy.iter().for_each(|energy| {
        let year = energy.date.split('-').next().unwrap_or("").to_string();
        *production_by_years.entry(year).or_insert(0.0) += energy.production;
    });

    production_by_years.iter().for_each(|(year, production)| {
        println!("{}: {} GWh", year, production);
    });

    println!("-----------------------------------------------------------------------------------------------------");


    all_energy
        .iter()
        .into_grouping_map_by(|element| element.date.split('-').next().unwrap_or("").to_string())
        .fold(0.0, |acc, _key, elements| acc + elements.production)
        .iter()
        .sorted_by(|a, b| a.0.parse::<u32>().unwrap().cmp(&b.0.parse::<u32>().unwrap()))
        .for_each(|total| println!("Total production for {}, {} Gwh", total.0, total.1));


    println!("-----------------------------------------------------------------------------------------------------");


    // Calculate production by months
    let mut production_by_months: HashMap<String, f64> = HashMap::new();

    all_energy
        .iter()
        .map(|energy| {
            let month = energy.date.split('-').nth(1).unwrap_or("").to_string();
            (month, energy.production)
        })
        .for_each(|(month, production)| {
            let months = production_by_months.entry(month).or_insert(0.0);
            *months += production;
        });

    production_by_months.iter().for_each(|(month, production)| {
        println!("{}: {} GWh", month, production);
    });

    println!("------------------------------------------------------------------------------------------------------");

     all_energy
        .iter()
        .into_grouping_map_by(|element| element.date.split('-').nth(1).unwrap_or("").to_string())
        .fold(0.0, |acc, _key, elements| acc + elements.production)
        .iter()
        .sorted_by(|a, b| a.0.parse::<u32>().unwrap().cmp(&b.0.parse::<u32>().unwrap()))
        .for_each(|total| println!("Total production for {}, {} Gwh", total.0, total.1));

    println!("-----------------------------------------------------------------------------------------------------");


    //Type of energy that produced the most all years combined

    let most_productive = production_by_type
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal));

    match most_productive {
        Some((most_productive_energy_type, most_production)) => println!(
            "The type of energy that produced the most across all years combined is {} with {} GWh",
            most_productive_energy_type, most_production
        ),
        None => print!("Aucunne énergie productive"),
    }

    println!("-----------------------------------------------------------------------------------------------------");


    //Year with the most production

    if let Some((most_productive_year, most_production)) = production_by_years
        .iter()
        .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
    {
        println!(
            "The year with the most production is {} with {} GWh",
            most_productive_year, most_production
        );
    }

    Ok(())
}
