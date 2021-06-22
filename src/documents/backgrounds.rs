use serde::{Deserialize, Serialize};
use crate::documents::{Alignment, Choice, EquipmentCurrent, EquipmentCategory,
                       Info, Language, Proficiency};

#[derive(Serialize, Deserialize, Clone)]
pub struct Background {
    pub index: String,
    pub name: String,
    pub starting_proficiencies: Vec<Proficiency>,
    pub language_options: Choice<Language>,
    pub starting_equipment: Vec<EquipmentCurrent>,
    //pub starting_equipment_options: Vec<EquipmentCategory>,
    pub feature: Info,
    pub personality_traits: Choice<String>,
    pub ideals: Choice<Ideal>,
    pub bonds: Choice<String>,
    pub flaws: Choice<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Ideal {
    pub desc: String,
    pub alignments: Vec<Alignment>,
}
