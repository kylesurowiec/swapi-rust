extern crate reqwest;

#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod query;
pub mod types;

use self::types::people::People;
use self::types::planets::Planet;
