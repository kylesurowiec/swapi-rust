#[derive(Debug, Deserialize, Default)]
pub struct People {
    name: String,
    birth_year: String,
    eye_color: String,
    gender: String,
    hair_color: String,
    height: String,
    mass: String,
    skin_color: String,
    homeworld: String,
    films: Vec<String>,
    species: Vec<String>,
    starships: Vec<String>,
    vehicles: Vec<String>,
    url: String,
    created: String,
    edited: String,
}

pub fn query_people(people_num: &str) {
    // Base URL for a people request
    let base_url: String = "/people/".to_owned();
    let people_url: &str = &(base_url + &people_num);

    let results = super::query::api_query(people_url);
    match results {
        Ok(mut r) => println!("{:#?}", r.json::<People>()),
        Err(e) => println!("{:#?}", e),
    }
}
