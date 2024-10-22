mod prelude;
mod tmx;

use tmx::Map;

fn main() {
    let map = Map::load("assets/example.tmx")
        .expect("Failed to read file");

    println!("{:?}", map);
}
