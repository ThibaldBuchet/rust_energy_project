use std::fs::File;
use std::io::prelude::*;
use std::error::Error;
use serde::Deserialize;


fn main() {
  
  let mut file =File::open("ogd104_stromproduktion_swissgrid.csv").expect("Can't open the file!");

  let mut contents = String::new();
  file.read_to_string(&mut contents)
        .expect("Can't read the file!");

    println!("File Contents:\n\n{}", contents);

    struct Energy{
        date: u64,
        energy_type: String,
        production: f64,
    }

    impl Energy {
        fn new(date: u64, energy_type: String, production: f64) -> Self{
            Self{
                date,
                energy_type,
                production,
            }
        }
        
    } 

    

}
