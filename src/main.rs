extern crate swapi;

fn main() {
    swapi::types::starships::query_starships("3");
}
