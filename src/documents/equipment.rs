use serde::{Deserialize, Serialize};
use crate::documents::{Damage, EquipmentCategory, WeaponProperty};

#[derive(Serialize, Deserialize, Clone)]
pub struct Equipment {
    pub index: String,
    pub name: String,
    pub equipment_category: Option<EquipmentCategory>,
    // Weapons
    pub weapon_category: Option<String>,
    pub weapon_range: Option<String>,
    pub category_range: Option<String>,
    pub damage: Option<Damage>,
    pub range: Option<Range>,
    pub throw_range: Option<Range>,
    pub properties: Option<Vec<WeaponProperty>>,
    // Armor
    pub armor_category: Option<String>,
    pub armor_class: Option<ArmorClass>,
    pub str_minimum: Option<i32>,
    pub stealth_disadvantage: Option<bool>,
    // Adventuring gear
    pub gear_category: Option<EquipmentCategory>,
    pub quantity: Option<i32>,
    // Tools
    pub tool_category: Option<String>,
    // Tools
    pub vehicle_category: Option<String>,
    pub speed: Option<Cost>,
    pub capacity: Option<String>,
    // General
    pub cost: Option<Cost>,
    pub weight: Option<f32>,
    pub desc: Option<Vec<String>>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Range {
    pub normal: i32,
    pub long: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ArmorClass {
    pub base: i32,
    pub dex_bonus: bool,
    pub max_bonus: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Cost {
    pub quantity: f32,
    pub unit: String,
}
