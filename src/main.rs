use tiled::Loader;

mod prelude;
mod soil;

fn main() {
    let mut loader = Loader::new();
    let map = loader
        .load_tmx_map("assets/example.tmx")
        .expect("Failed to read file");

    assert_eq!(map.infinite(), false, "Infinite maps are not supported.");
    // println!("{:#?}", map.layers().collect::<Vec<_>>());

    let soil = soil::SpiderOil::create_from_tiled_map(&map);
    let sexpr = soil.s_expr();

    println!("{}", sexpr);

    let sexprtest = lexpr::from_str(&sexpr);
    println!("DEBUG:\n{:#?}", sexprtest);
}
