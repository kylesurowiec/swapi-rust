extern crate swapi;

fn main() {
    let mut planet_resp: swapi::types::Species = Default::default();
    swapi::query_species("6", &mut planet_resp);
    println!("{:#?}", planet_resp);
}
