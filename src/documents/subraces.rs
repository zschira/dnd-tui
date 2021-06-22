use serde::{Deserialize, Serialize};
use crate::documents::{AbilityBonus, Language, Proficiency, Race, Trait};

#[derive(Serialize, Deserialize, Clone)]
pub struct Subrace {
    pub index: String,
    pub name: String,
    pub race: Option<Race>,
    pub desc: Option<String>,
    pub ability_bonuses: Option<Vec<AbilityBonus>>,
    pub starting_proficiencies: Option<Vec<Proficiency>>,
    pub languages: Option<Vec<Language>>,
    pub racial_traits: Option<Vec<Trait>>,
    pub url: String,
}
