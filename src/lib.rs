extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod types;

use self::types::*;

pub fn api_query(endpoint: &str) -> Result<reqwest::Response, reqwest::Error> {
    // Base URL for all API requests
    // endpoint is concatenated onto base_url
    let base_url: String = "https://swapi.co/api".to_owned();
    let query_url: &str = &(base_url + &endpoint);

    // client should be reused to make use of internal connection pool
    let client = reqwest::Client::new();

    let resp = client.get(query_url).send();
    let query_results: Result<reqwest::Response, reqwest::Error> = match resp {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
    };

    query_results
}

pub fn query_film(title: &str, _film_buf: &mut Film) { 
    let base_url: String = "/films/".to_owned();
    let film_url: &str = &(base_url + &title);

    let results = api_query(film_url);
    match results {
        Ok(mut r) => {
            *_film_buf = match r.json::<Film>() {
                Ok(v) => v,
                Err(e) => panic!("Decoding error {:#?}", e)
            }.clone();

        },
        Err(e) => panic!("{:#?}", e),
    }
}

pub fn query_people(people_num: &str, _people_buf: &mut People) {
    // Base URL for a people request
    let base_url: String = "/people/".to_owned();
    let people_url: &str = &(base_url + &people_num);
    let results = api_query(people_url);
    match results {
        Ok(mut r) => {
            *_people_buf = match r.json::<People>() {
                Ok(v) => v,
                Err(e) => panic!("Decoding error {:#?}", e)
            }.clone();

        },
        Err(e) => panic!("{:#?}", e),
    }
}

pub fn query_planet(planet_num: &str, _planet_buf: &mut Planet) {
    // Base URL for a planet request
    let base_url: String = "/planets/".to_owned();
    let planet_url: &str = &(base_url + &planet_num);
    let results = api_query(planet_url);
    match results {
        Ok(mut r) => {
            *_planet_buf = match r.json::<Planet>() {
                Ok(v) => v,
                Err(e) => panic!("Decoding error {:#?}", e)
            }.clone();

        },
        Err(e) => panic!("{:#?}", e),
    }
}

pub fn query_species(species_num: &str, _species_buf: &mut Species) {
    // Base URL for a species request
    let base_url: String = "/species/".to_owned();
    let species_url: &str = &(base_url + &species_num);
    let results = api_query(species_url);
    match results {
        Ok(mut r) => {
            *_species_buf = match r.json::<Species>() {
                Ok(v) => v,
                Err(e) => panic!("Decoding error {:#?}", e)
            }.clone();

        },
        Err(e) => panic!("{:#?}", e),
    }
}

pub fn query_starships(starships_num: &str, _starships_buf: &mut Starships) {
    // Base URL for a starships request
    let base_url: String = "/starships/".to_owned();
    let starships_url: &str = &(base_url + &starships_num);
    let results = api_query(starships_url);
    match results {
        Ok(mut r) => {
            *_starships_buf = match r.json::<Starships>() {
                Ok(v) => v,
                Err(e) => panic!("Decoding error {:#?}", e)
            }.clone();

        },
        Err(e) => panic!("{:#?}", e),
    }
}

pub fn query_vehicles(vehicles_num: &str, _vehicles_buf: &mut types::Vehicles) {
    // Base URL for a vehicles request
    let base_url: String = "/vehicles/".to_owned();
    let vehicles_url: &str = &(base_url + &vehicles_num);

    let results = api_query(vehicles_url);
    match results {
        Ok(mut r) => {
            *_vehicles_buf = match r.json::<types::Vehicles>() {
                Ok(v) => v,
                Err(e) => panic!("")
            }.clone();
        },
        Err(e) => println!("{:#?}", e),
    }
}






/* 
 * serde_derive & generics
 *
 * Generic query method, not (fully) functioning due to the unique implemententations
 * generated for each struct when Deserialize is derived (serde_derive). 
 * 
 * When a struct derives Deserialize, serde_derive generates a unique implementation 
 * for that struct, its implementation is generated in a module with this pattern:
 *  CURR_MOD::_IMPL_DESERIALIZE_FOR_StructName::_serde::Deserialize<'de>
 * 
 * Rust doesnt accept the uniquly generated implementations as satisfying the
 * where T: serde::Deserialize type requirement.
 *
 * I believe it could be worked around by manually implementing Deserialize, or by
 * taking a look at the macros serde provides to see if theres a way to
 * generate Deserialize implementations that can satisfy type requirements.
 *
 * Future release candidate
 *
 *
pub fn query<'de, T>(endpoint: &str, _type_buf: &mut T) 
    where T: serde::de::Deserialize<'de> + Default + Clone {
    let results = api_query(endpoint);
    match results {
        Ok(mut r) => {
            *_type_buf = match r.json::<T>() {
                Ok(v) => v,
                Err(e) => panic!("Fatal Decoding Error {:#?}", e)
            }.clone();
        },
        Err(e) => panic!("{:#?}", e)
    }
}
*/
