#[derive(Debug, Deserialize, Default, Clone)]
pub struct Planet {
    climate: String,
    created: String,
    diameter: String,
    edited: String,
    films: Vec<String>,
    gravity: String,
    name: String,
    orbital_period: String,
    population: String,
    residents: Vec<String>,
    rotation_period: String,
    surface_water: String,
    terrain: String,
    url: String,
}

pub fn query_planet(planet_num: &str, _planet_buf: &mut Planet) {
    // Base URL for a planet request
    let base_url: String = "/planets/".to_owned();
    let planet_url: &str = &(base_url + &planet_num);
    let results = super::query::api_query(planet_url);
    match results {
        Ok(mut r) => {
            *_planet_buf = match r.json::<Planet>() {
                Ok(v) => v,
                Err(e) => panic!("Decoding error {:#?}", e)
            }.clone();

        },
        Err(e) => panic!("{:#?}", e),
    }
}
