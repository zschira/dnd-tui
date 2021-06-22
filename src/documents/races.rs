use serde::{Deserialize, Serialize};
use crate::documents::{AbilityBonus, Choice, Language, Proficiency, Subrace, Trait};

#[derive(Serialize, Deserialize, Clone)]
pub struct Race {
    pub index: String,
    pub name: String,
    pub speed: Option<i32>,
    pub ability_bonuses: Option<Vec<AbilityBonus>>,
    pub alignment: Option<String>,
    pub age: Option<String>,
    pub size: Option<String>,
    pub size_description: Option<String>,
    pub starting_proficiencies: Option<Vec<Proficiency>>,
    pub starting_proficiency_options: Option<Choice<Proficiency>>,
    pub languages: Option<Vec<Language>>,
    pub language_desc: Option<String>,
    pub traits: Option<Vec<Trait>>,
    pub subraces: Option<Vec<Subrace>>,
    pub url: String,
}
