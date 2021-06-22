use serde::{Deserialize, Serialize};
use crate::documents::{AbilityScore, Choice, EquipmentCurrent, 
                       Info, Proficiency, Subclass};

#[derive(Serialize, Deserialize, Clone)]
pub struct Class {
    pub index: String,
    pub name: String,
    pub hit_die: Option<i32>,
    pub proficiency_choices: Option<Vec<Choice<Proficiency>>>,
    pub proficiencies: Option<Vec<Proficiency>>,
    pub saving_throws: Option<Vec<AbilityScore>>,
    pub starting_equipment: Option<Vec<EquipmentCurrent>>,
    //pub starting_equipment_options: Option<Vec<Choice<EquipmentCurrent>>>,
    pub class_levels: Option<String>,
    pub subclasses: Option<Vec<Subclass>>,
    pub spellcasting: Option<Spellcasting>,
    pub spells: Option<String>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Spellcasting {
    pub level: i32,
    pub spellcasting_ability: AbilityScore,
    pub info: Vec<Info>,
    //pub url: String,
}
