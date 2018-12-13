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
