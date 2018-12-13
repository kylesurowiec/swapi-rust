extern crate swapi;

fn main() {
    let mut planet_resp: swapi::types::planets::Planet = Default::default();
    println!("{:#?}", planet_resp);
    swapi::query::query<swapi::types::planets::Planet>("/planets/1", &mut planet_resp);
    println!("{:#?}", planet_resp);
}
