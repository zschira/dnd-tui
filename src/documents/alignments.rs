use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Alignment {
    index: String,
    name: String,
    abbreviation: Option<String>,
    desc: Option<String>,
    url: String,
}
