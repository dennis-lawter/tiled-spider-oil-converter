use std::fs;
use std::path::Path;

use serde::Deserialize;
use serde::Serialize;

use serde_xml_rs::from_str;

use crate::prelude::*;

use super::layer::Layer;
use super::orientation::Orientation;
use super::renderorder::RenderOrder;
use super::tileset::TileSet;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Map {
    version: String,
    orientation: Orientation,
    renderorder: RenderOrder,
    width: u32,
    height: u32,
    tilewidth: u32,
    tileheight: u32,
    infinite: bool,
    backgroundcolor: String,
    // nextlayerid: u32,
    // nextobjectid: u32,
    #[serde(rename = "tileset")]
    tilesets: Vec<TileSet>,
    #[serde(rename = "layer")]
    layers: Vec<Layer>,
}
impl Map {
    pub fn load(path: &str) -> Result<Self, TmxError> {
        let path = Path::new(path);
        let dir = path.parent().ok_or(TmxError::DirectoryAccessError)?;
        println!("{:#?}", dir);
        let tmx = fs::read_to_string(path)?;
        let map: Map = from_str(&tmx).map_err(|e| TmxError::TmxMalformed(e))?;

        let expected_version = "1.10";

        if map.version != expected_version {
            println!(
                "WARNING: Map version {} may not be supported by this tool. Expected version: {}",
                map.version, expected_version
            );
        }

        assert_eq!(
            map.orientation,
            Orientation::Orthogonal,
            "FEATURE NOT SUPPORTED: orientations other than orthogonal"
        );
        assert_eq!(map.infinite, false, "FEATURE NOT SUPPORTED: infinite map");
        assert_eq!(
            map.renderorder,
            RenderOrder::RightDown,
            "FEATURE NOT SUPPORTED: renderorder other than rightdown"
        );

        Ok(map)
    }
}
