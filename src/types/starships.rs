#[derive(Debug, Deserialize, Default)]
pub struct Starships {
    name: String,
    model: String,
    starship_class: String,
    manufacturer: String,
    cost_in_credits: String,
    length: String,
    crew: String,
    passengers: String,
    max_atmosphering_speed: String,
    hyperdrive_rating: String,
    #[serde(rename = "MGLT")]
    mglt: String,
    cargo_capacity: String,
    consumables: String,
    films: Vec<String>,
    pilots: Vec<String>,
    url: String,
    edited: String,
}

pub fn query_starships(starships_num: &str) {
    // Base URL for a starships request
    let base_url: String = "/starships/".to_owned();
    let starships_url: &str = &(base_url + &starships_num);

    super::query::api_query(starships_url);
}
