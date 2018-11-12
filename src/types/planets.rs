#[derive(Debug, Deserialize, Default)]
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

pub fn query_planet(planet_num: &str) {
    // Base URL for a planet request
    let base_url: String = "/planets/".to_owned();
    let planet_url: &str = &(base_url + &planet_num);

    super::query::api_query(planet_url);
}
