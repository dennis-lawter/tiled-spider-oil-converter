pub struct Layer {
    pub data: Vec<u32>,
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
        let mut data: Vec<u32> = Vec::with_capacity(capacity);

        for y in 0..height {
            for x in 0..width {
                match layer.get_tile(x as i32, y as i32) {
                    Some(tile) => data.push(tile.id() + 1),
                    None => data.push(0u32),
                }
            }
        }

        Self { data }
    }
}
