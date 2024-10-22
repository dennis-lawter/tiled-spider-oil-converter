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

    pub fn s_expr(&self) -> String {
        let mut s_expr = String::new();

        for line in self.header.lines() {
            s_expr.push_str(&format!(";{}\n", line));
        }

        s_expr.push_str("(\n");
        {
            s_expr.push_str(&format!(" :width {}\n", self.width));
            s_expr.push_str(&format!(" :height {}\n", self.height));

            s_expr.push_str(" :layers\n");
            s_expr.push_str(" (\n");
            {
                for layer in &self.layers {
                    s_expr.push_str("  (\n   ");
                    for datum in &layer.data {
                        s_expr.push_str(&format!("{} ", datum));
                    }
                    s_expr.push_str("\n  )\n");
                }
            }
            s_expr.push_str(" )\n");

            s_expr.push_str(" :entities\n");
            s_expr.push_str(" (\n");
            {
                for entity in &self.entities {
                    s_expr.push_str("  (\n");
                    s_expr.push_str(&format!("   :x {}\n", entity.x));
                    s_expr.push_str(&format!("   :y {}\n", entity.y));
                    s_expr.push_str(&format!("   :name {}\n", entity.name));
                    s_expr.push_str(&format!("   :class {}\n", entity.class));
                    for (prop_name, prop_value) in &entity.properties {
                        s_expr.push_str(&format!("   :{} {}\n", prop_name, prop_value));
                    }
                    s_expr.push_str("  )\n");
                }
            }
            s_expr.push_str(" )\n");
        }
        s_expr.push_str(")\n");

        s_expr
    }
}
