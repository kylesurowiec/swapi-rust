// Possible Star Wars data types
// that a GET request can return
#[derive(Debug, Deserialize)]
pub enum StarWarsType {
    People,
    Films,
    Starships,
    Vehicles,
    Species,
    Planets(super::Planet),
}

pub fn api_query() {
    // Base URL for all API requests
    // let base_url: String = "https://swapi.co/api/".to_string();

    let mut response =
        reqwest::get("https://swapi.co/api/planets/1").expect("Failed to send request!");

    println!("{:#?}", response.json::<super::Planet>());
}
