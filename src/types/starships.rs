#[derive(Debug, Deserialize, Default, Clone)]
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

pub fn query_starships(starships_num: &str, _starships_buf: &mut Starships) {
    // Base URL for a starships request
    let base_url: String = "/starships/".to_owned();
    let starships_url: &str = &(base_url + &starships_num);

    let results = super::query::api_query(starships_url);
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
