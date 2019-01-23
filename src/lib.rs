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

/// Query films details given the film number.
///
/// # Examples
///
/// ```
/// swapi::query_film("1");
/// ```
pub fn query_film(title: &str) {
    let base_url: String = "/films/".to_owned();
    let film_url: &str = &(base_url + &title);

    let results = api_query(film_url);
    match results {
        Ok(mut r) => {
            match r.json::<Film>() {
                Ok(v) => println!("{:#?}", v),
                Err(e) => panic!("Decoding error {:#?}", e),
            };
        }
        Err(e) => panic!("{:#?}", e),
    }
}

/// Query a characters details given the characters number.
///
/// # Examples
///
/// ```
/// swapi::query_person("1");
/// ```
pub fn query_person(people_num: &str) {
    let base_url: String = "/people/".to_owned();
    let person_url: &str = &(base_url + &people_num);

    let results = api_query(person_url);
    // let mut _people_buf: self::types::Person = Default::default();

    match results {
        Ok(mut r) => match r.json::<Person>() {
            Ok(v) => println!("{:#?}", v),
            Err(e) => panic!("Decoding error {:#?}", e),
        },
        Err(e) => panic!("{:#?}", e),
    }
}

/// Query a planets details given the planet number.
///
/// # Examples
///
/// ```
/// swapi::query_planet("1");
/// ```
pub fn query_planet(planet_num: &str) {
    let base_url: String = "/planets/".to_owned();
    let planet_url: &str = &(base_url + &planet_num);

    let results = api_query(planet_url);
    // let mut _planet_buf: self::types::Planet = Default::default();

    match results {
        Ok(mut r) => {
            match r.json::<Planet>() {
                Ok(v) => println!("{:#?}", v),
                Err(e) => panic!("Decoding error {:#?}", e),
            };
        }
        Err(e) => panic!("{:#?}", e),
    }
}

/// Query a species details given the species number.
///
/// # Examples
///
/// ```
/// swapi::query_species("1");
/// ```
pub fn query_species(species_num: &str) {
    let base_url: String = "/species/".to_owned();
    let species_url: &str = &(base_url + &species_num);

    let results = api_query(species_url);
    // let mut _species_buf: self::types::Planet = Default::default();

    match results {
        Ok(mut r) => {
            match r.json::<Species>() {
                Ok(v) => println!("{:#?}", v),
                Err(e) => panic!("Decoding error {:#?}", e),
            };
        }
        Err(e) => panic!("{:#?}", e),
    }
}

/// Query a starships details given the starship number.
///
/// # Examples
///
/// ```
/// swapi::query_starship("1");
/// ```
pub fn query_starship(starship_num: &str) {
    let base_url: String = "/starships/".to_owned();
    let starship_url: &str = &(base_url + &starship_num);

    let results = api_query(starship_url);
    // let mut _starship_buf: self::types::Planet = Default::default();

    match results {
        Ok(mut r) => {
            match r.json::<Starship>() {
                Ok(v) => println!("{:#?}", v),
                Err(e) => panic!("Decoding error {:#?}", e),
            };
        }
        Err(e) => panic!("{:#?}", e),
    }
}

/// Query a vehicles details given the vehicle number.
///
/// # Examples
///
/// ```
/// swapi::query_starship("1");
/// ```
pub fn query_vehicle(vehicle_num: &str) {
    let base_url: String = "/vehicles/".to_owned();
    let vehicle_url: &str = &(base_url + &vehicle_num);

    let results = api_query(vehicle_url);
    // let mut _starship_buf: self::types::Planet = Default::default();

    match results {
        Ok(mut r) => {
            match r.json::<types::Vehicle>() {
                Ok(v) => println!("{:#?}", v),
                Err(e) => panic!("Decoding error {:#?}", e),
            };
        }
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
