#[derive(Debug, Deserialize, Default, Clone)]
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

pub fn query_film(title: &str, _film_buf: &mut Film) { 
    let base_url: String = "/films/".to_owned();
    let film_url: &str = &(base_url + &title);

    let results = super::query::api_query(film_url);
    match results {
        Ok(mut r) => {
            *_film_buf = match r.json::<Film>() {
                Ok(v) => v,
                Err(e) => panic!("Decoding error {:#?}", e)
            }.clone();

        },
        Err(e) => panic!("{:#?}", e),
    }
}
