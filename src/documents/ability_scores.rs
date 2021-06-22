use serde::{Deserialize, Serialize};
use crate::documents::Skill;

#[derive(Serialize, Deserialize, Clone)]
pub struct AbilityScore {
    pub index: String,
    pub name: String,
    pub full_name: Option<String>,
    pub desc: Option<Vec<String>>,
    pub skills: Option<Vec<Skill>>,
    pub url: String,
}
