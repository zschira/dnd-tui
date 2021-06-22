use serde::{Deserialize, Serialize};
use crate::documents::{Class, Race};

#[derive(Serialize, Deserialize, Clone)]
pub struct Proficiency {
    pub index: String,
    #[serde(rename = "type")]
    pub prof_type: Option<String>,
    pub name: String,
    pub classes: Option<Vec<Class>>,
    pub races: Option<Vec<Race>>,
    pub url: String,
}
