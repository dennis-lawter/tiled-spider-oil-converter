use std::fmt;

pub struct Tile {
    pub flip_v: bool,
    pub flip_h: bool,
    pub flip_d: bool,
    pub tile_set_name: String,
    pub tile_id: u32,
}
impl Tile {
    pub fn from_tiled_tile(tile: &tiled::LayerTile) -> Self {
        let flip_v = tile.flip_v;
        let flip_h = tile.flip_h;
        let flip_d = tile.flip_d;
        let tile_set_name = tile.get_tileset().name.to_owned();
        let tile_id = tile.id();
        Self {
            flip_v,
            flip_h,
            flip_d,
            tile_set_name,
            tile_id,
        }
    }
    pub fn get_translation_code(&self) -> String {
        match (self.flip_h, self.flip_v, self.flip_d) {
            (true, true, true) | (true, true, false) => " :t 270".to_owned(),
            (true, false, true) => " :t 90".to_owned(),
            (true, false, false) => " :t h".to_owned(),
            (false, true, true) => " :t 180".to_owned(),
            (false, true, false) => " :t v".to_owned(),
            (false, false, true) => " :t v90".to_owned(),
            (false, false, false) => "".to_owned(),
        }
    }
}
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({} {}{})",
            self.tile_set_name,
            self.tile_id,
            self.get_translation_code()
        )
    }
}

pub struct Layer {
    pub tiles: Vec<Option<Tile>>,
}
impl Layer {
    pub fn create_all_from_tiled_map(map: &tiled::Map) -> Vec<Self> {
        let mut layers = vec![];

        let width = map.width;
        let height = map.height;

        for layer in map.layers() {
            match layer.as_tile_layer() {
                Some(tile_layer) => {
                    layers.push(Self::create_one_from_tiled_layer(
                        &tile_layer,
                        width,
                        height,
                    ));
                }
                None => {}
            }
        }

        layers
    }

    fn create_one_from_tiled_layer(layer: &tiled::TileLayer, width: u32, height: u32) -> Layer {
        let capacity = width as usize * height as usize;
        let mut tiles: Vec<Option<Tile>> = Vec::with_capacity(capacity);

        for y in 0..height {
            for x in 0..width {
                match layer.get_tile(x as i32, y as i32) {
                    Some(tile) => tiles.push(Some(Tile::from_tiled_tile(&tile))),
                    _ => tiles.push(None),
                }
            }
        }

        Self { tiles }
    }
}
