extern crate swapi;

fn main() {
    swapi::query_species("6");
    println!("{:#?}", planet_resp);
}
