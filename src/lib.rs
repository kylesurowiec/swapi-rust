extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod query;
pub mod types;

use self::types::films::Film;
use self::types::people::People;
use self::types::planets::Planet;
use self::types::species::Species;
use self::types::starships::Starships;
use self::types::vehicles::Vehicles;
