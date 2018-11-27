#[derive(Debug, Deserialize, Default)]
pub struct Species {
    average_height: String,
    average_lifespan: String,
    classification: String,
    created: String,
    designation: String,
    edited: String,
    eye_colors: String,
    hair_colors: String,
    homeworld: String,
    language: String,
    name: String,
    people: Vec<String>,
    films: Vec<String>,
    skin_colors: String,
    url: String,
}

pub fn query_species(species_num: &str) {
    // Base URL for a species request
    let base_url: String = "/species/".to_owned();
    let species_url: &str = &(base_url + &species_num);

    let results = super::query::api_query(species_url);
    match results {
        Ok(mut r) => println!("{:#?}", r.json::<Species>()),
        Err(e) => println!("{:#?}", e),
    }
}
