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

pub fn query_planet() {
    super::query::api_query();
}
