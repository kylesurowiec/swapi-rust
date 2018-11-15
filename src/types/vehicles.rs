#[derive(Debug, Deserialize, Default)]
pub struct Vehicles {
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

pub fn query_vehicles(vehicles_num: &str) {
    // Base URL for a vehicles request
    let base_url: String = "/vehicles/".to_owned();
    let vehicles_url: &str = &(base_url + &vehicles_num);

    super::query::api_query(vehicles_url);
}
