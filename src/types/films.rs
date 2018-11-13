#[derive(Debug, Deserialize, Default)]
pub struct Film {
    title: String,
    episode_id: i32,
    opening_crawl: String,
    director: String,
    producer: String,
    release_date: String, // ISO 8601 of release date
    species: Vec<String>,
    starships: Vec<String>,
    vehicles: Vec<String>,
    characters: Vec<String>,
    planets: Vec<String>,
    url: String,
    created: String,
    edited: String,
}

pub fn query_film(title: &str) {
    let base_url: String = "/films/".to_owned();
    let film_url: &str = &(base_url + &title);

    super::query::api_query(film_url);
}
