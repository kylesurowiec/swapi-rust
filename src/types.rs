// Possible Star Wars data types
// that a GET request can return
// #[derive(Debug, Deserialize)]
// pub enum StarWarsType {
//     People(Person),
//     Films(Film),
//     Starships(Starship),
//     Vehicles(Vehicle),
//     Species(Species),
//     Planets(Planet),
// }

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Film {
    title: String,
    episode_id: i32,
    opening_crawl: String,
    director: String,
    producer: String,
    release_date: String,
    species: Vec<String>,
    starships: Vec<String>,
    vehicles: Vec<String>,
    characters: Vec<String>,
    planets: Vec<String>,
    url: String,
    created: String,
    edited: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Person {
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

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Species {
    average_height: String,
    average_lifespan: String,
    classification: String,
    created: String,
    designation: String,
    edited: String,
    eye_colors: String,
    hair_colors: String,
    homeworld: Option<String>,
    language: String,
    name: String,
    people: Vec<String>,
    films: Vec<String>,
    skin_colors: String,
    url: String,
}

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Starship {
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

#[derive(Debug, Deserialize, Default, Clone)]
pub struct Vehicle {
    cargo_capacity: String,
    consumables: String,
    cost_in_credits: String,
    created: String,
    crew: String,
    edited: String,
    length: String,
    manufacturer: String,
    max_atmosphering_speed: String,
    model: String,
    name: String,
    passengers: String,
    pilots: Vec<String>,
    films: Vec<String>,
    url: String,
    vehicle_class: String,
}
