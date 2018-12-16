extern crate swapi;

fn main() {
    let mut planet_resp: swapi::types::species::Species = Default::default();
    swapi::types::species::query_species("6", &mut planet_resp);
    println!("{:#?}", planet_resp);
}
