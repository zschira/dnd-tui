use serde::{Deserialize, Serialize};
use crate::documents::{Proficiency, Race, Subrace};

#[derive(Serialize, Deserialize, Clone)]
pub struct Trait {
    pub index: String,
    pub name: String,
    pub races: Option<Vec<Race>>,
    pub subraces: Option<Vec<Subrace>>,
    pub desc: Option<Vec<String>>,
    pub proficiencies: Option<Vec<Proficiency>>,
    pub url: String,
}
