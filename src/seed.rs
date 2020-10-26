use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;

use db::{airplanes, pg_pool};

fn main() {
    let path = Path::new("/Users/rosstodd/Projects/rust/rusti_aircraft_api/src/bin/airplanes.json");
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");
    println!("{}", contents);

    diesel::insert_into(&contents)
        .into(airplanes)
        .execute(&*connection)
        .expect("Error inserting airplanes");


    Airplane::create()
}
