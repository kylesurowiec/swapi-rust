// Possible Star Wars data types
// that a GET request can return
#[derive(Debug, Deserialize)]
pub enum StarWarsType {
    People(super::People),
    Films(super::Film),
    Starships(/*super::Starships*/),
    Vehicles(/*super::Vehicles*/),
    Species(/*super::Species*/),
    Planets(super::Planet),
}

pub fn api_query(endpoint: &str) {
    // Base URL for all API requests
    let base_url: String = "https://swapi.co/api".to_owned();
    // Concatenate endpoint onto base_url
    let query_url: &str = &(base_url + &endpoint);

    let mut res = reqwest::get(query_url).expect("Failed request!");

    if !res.status().is_success() {
        println!("Error with request - {}({})", query_url, res.status());
        panic!();
    }

    let data = res.json::<super::Planet>();

    match data {
        Ok(s) => println!("{:#?}", s),
        Err(e) => println!("{:#?}", e),
    }
}
