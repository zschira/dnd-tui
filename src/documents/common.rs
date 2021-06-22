use serde::{Deserialize, Serialize};
use crate::documents::{AbilityScore, Equipment, DamageType};

#[derive(Serialize, Deserialize, Clone)]
pub struct Dc {
    pub dc_type: AbilityScore,
    pub dc_success: Option<String>,
    pub dc_value: Option<i32>,
    pub success_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Choice<T> {
    pub choose: i32,
    #[serde(rename = "type")]
    pub choice_type: String,
    pub from: Vec<T>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Damage {
    pub damage_dice: Option<String>,
    pub damage_type: Option<DamageType>,
    pub damage_at_slot_level: Option<DamageAtSlotLevel>,
    pub damage_at_character_level: Option<DamageAtCharacterLevel>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DamageAtSlotLevel {
    #[serde(rename = "2")]
    pub two: Option<String>,
    #[serde(rename = "3")]
    pub three: Option<String>,
    #[serde(rename = "4")]
    pub four: Option<String>,
    #[serde(rename = "5")]
    pub five: Option<String>,
    #[serde(rename = "6")]
    pub six: Option<String>,
    #[serde(rename = "7")]
    pub seven: Option<String>,
    #[serde(rename = "8")]
    pub eight: Option<String>,
    #[serde(rename = "9")]
    pub nine: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DamageAtCharacterLevel {
    #[serde(rename = "1")]
    pub one: String,
    #[serde(rename = "5")]
    pub five: String,
    #[serde(rename = "11")]
    pub eleven: String,
    #[serde(rename = "17")]
    pub seventeen: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EquipmentCurrent {
    pub equipment: Equipment,
    pub quantity: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Info {
    pub name: String,
    pub desc: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AbilityBonus {
    ability_score: AbilityScore,
    bonus: i32,
}
