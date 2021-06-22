use serde::{Deserialize, Serialize};
use crate::documents::AbilityScore;

#[derive(Serialize, Deserialize, Clone)]
pub struct Skill {
    pub index: String,
    pub name: String,
    pub desc: Option<Vec<String>>,
    pub ability_score: Option<AbilityScore>,
    pub url: String,
}
