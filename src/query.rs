// Possible Star Wars data types
// that a GET request can return
#[derive(Debug, Deserialize)]
pub enum StarWarsType {
    People(super::people::People),
    Films(super::films::Film),
    Starships(super::starships::Starships),
    Vehicles(super::vehicles::Vehicles),
    Species(super::species::Species),
    Planets(super::planets::Planet),
}

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
