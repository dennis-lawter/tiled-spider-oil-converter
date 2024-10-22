use std::collections::BTreeMap;

pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub name: String,
    pub class: String,
    pub properties: BTreeMap<String, String>,
}
impl Entity {
    pub fn create_all_from_tiled_map(map: &tiled::Map) -> Vec<Self> {
        let mut objs = vec![];

        for layer in map.layers() {
            match layer.as_object_layer() {
                Some(obj_layer) => {
                    for obj in obj_layer.objects() {
                        let x = obj.x;
                        let y = obj.y;
                        let name = obj.name.to_owned();
                        let class = obj.user_type.to_owned();
                        let mut properties = BTreeMap::<String, String>::new();
                        for (prop_name, prop_value) in &obj.properties {
                            // TODO: actually serialize values
                            properties.insert(prop_name.to_owned(), format!("{:?}", prop_value));
                        }
                        objs.push(Self {
                            x,
                            y,
                            name,
                            class,
                            properties,
                        });
                    }
                }
                None => {}
            }
        }

        objs
    }
}
