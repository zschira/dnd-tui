use serde::{Deserialize, Serialize};
use crate::documents::{Class, Feature, Subclass};

#[derive(Serialize, Deserialize, Clone)]
pub struct Level {
    pub level: i32,
    pub ability_score_bonuses: Option<i32>,
    pub prof_bonus: Option<i32>,
    pub feature_choices: Vec<Feature>,
    pub features: Vec<Feature>,
    pub spellcasting: Option<Spellcasting>,
    pub index: String,
    pub class: Class,
    pub subclass: Option<Subclass>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Spellcasting {
    pub cantrips_known: Option<i32>,
    pub spell_slots_level_1: Option<i32>,
    pub spell_slots_level_2: Option<i32>,
    pub spell_slots_level_3: Option<i32>,
    pub spell_slots_level_4: Option<i32>,
    pub spell_slots_level_5: Option<i32>,
    pub spell_slots_level_6: Option<i32>,
    pub spell_slots_level_7: Option<i32>,
    pub spell_slots_level_8: Option<i32>,
    pub spell_slots_level_9: Option<i32>,
}
