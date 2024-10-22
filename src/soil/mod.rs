use entity::Entity;
use layer::Layer;

pub mod entity;
pub mod layer;

// pub struct Link {
//     x: f32,
//     y: f32,
//     url: String,
// }
// pub struct Tileset {
//     tiles: Vec<Tile>,
// }
// pub struct Tile {
//     data: Vec<u8>, // assuming base64 encoding
// }
pub struct SpiderOil {
    header: String,
    width: u32,
    height: u32,

    layers: Vec<Layer>,

    entities: Vec<Entity>,
}

impl SpiderOil {
    pub fn create_from_tiled_map(map: &tiled::Map) -> Self {
        let default_header = r#"<h1>Intended for the <a href="https://github.com/lcolonq/spideroil">Spider Oil browser</a></h1><pre>"#;
        let header_property_opt = map.properties.get("header");
        let header = match header_property_opt {
            Some(tiled::PropertyValue::StringValue(header_property)) => header_property.to_owned(),
            _ => default_header.to_owned(),
        };
        let width = map.width;
        let height = map.height;
        let layers = Layer::create_all_from_tiled_map(map);
        let entities = Entity::create_all_from_tiled_map(map);

        Self {
            header,
            width,
            height,
            layers,
            entities,
        }
    }

    pub fn print_s_expr(&self) {
        for line in self.header.lines() {
            println!(";{}", line);
        }
        println!("(");
        {
            println!(" :width {}", self.width);
            println!(" :height {}", self.height);

            println!(" :layers");
            println!(" (");
            {
                for layer in &self.layers {
                    println!("  (");
                    print!("   ");
                    for datum in &layer.data {
                        print!("{} ", datum);
                    }
                    println!("");
                    println!("  )");
                }
            }
            println!(" )");

            println!(" :entities");
            println!(" (");
            {
                for entity in &self.entities {
                    println!("  (");
                    println!("   :x {}", entity.x);
                    println!("   :y {}", entity.y);
                    println!("   :name {}", entity.name);
                    println!("   :class {}", entity.class);
                    for (prop_name, prop_value) in &entity.properties {
                        println!("   :{} {}", prop_name, prop_value);
                    }
                    println!("  )");
                }
            }
            println!(" )");
        }
        println!(")");
    }
}
