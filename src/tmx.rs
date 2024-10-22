use std::fs;

use serde::{Serialize, Deserialize};
use serde_xml_rs::from_str;

use crate::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub enum Orientation {
    #[serde(rename = "orthogonal")]
    Orthogonal,
    #[serde(rename = "isometric")]
    Isometric,
    #[serde(rename = "staggered")]
    Staggered,
    #[serde(rename = "hexagonal")]
    Hexagonal
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Layer {
    id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Map {
    version: String,
    orientation: Orientation,
    // renderorder: RenderOrder,
    width: u32,
    height: u32,
    tilewidth: u32,
    tileheight: u32,
    // infinite: bool,
    backgroundcolor: String,
    // nextlayerid: u32,
    // nextobjectid: u32,

    #[serde(rename = "layer")]
    layers: Vec<Layer>,
}
impl Map {
    pub fn load(path: &str) -> Result<Self, TmxError> {
        let tmx = fs::read_to_string(path)?;
        let map: Map = from_str(&tmx).map_err(|e|TmxError::TmxMalformed(e))?;

        Ok(map)
    }
}
