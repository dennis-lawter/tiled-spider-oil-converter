pub struct TileSet {
    pub name: String,
    pub url: String,
}
impl TileSet {
    pub(crate) fn create_all_from_tiled_map(map: &tiled::Map) -> Vec<Self> {
        let mut tilesets = vec![];
        for tileset in map.tilesets() {
            let name = tileset.name.to_owned();
            println!("{:#?}", tileset.properties);
            let url = tileset.properties.get("url").expect(&format!(
                "The **url** property must be set on tileset {}.",
                name
            ));
            let url = match url {
                tiled::PropertyValue::StringValue(url_string) => url_string.to_owned(),
                _ => {
                    panic!(
                        "The **url** property must be a valid string on tileset {}.",
                        name
                    );
                }
            };
            tilesets.push(TileSet { name, url });
        }

        tilesets
    }
}
