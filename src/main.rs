use std::fs;

use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Serialize, Deserialize)]
struct Map {
    version: String,
}

fn main() {
    let tmx = fs::read_to_string("assets/example.tmx")
    .expect("Failed to read");

    let tiled: Map = from_str(&tmx).unwrap();
    println!("{:?}", tiled);
}
