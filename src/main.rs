use std::fs::File;
use std::io::prelude::*;


fn main() {
  
  let mut file =File::open("ogd104_stromproduktion_swissgrid.csv").expect("Can't open the file!");

  let mut contents = String::new();
  file.read_to_string(&mut contents)
        .expect("Can't read the file!");

    println!("File Contents:\n\n{}", contents);

}
