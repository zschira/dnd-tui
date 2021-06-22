use serde::{Deserialize, Serialize};
use crate::documents::{Condition, Damage, Dc, Proficiency};

#[derive(Serialize, Deserialize, Clone)]
pub struct Monster {
    pub index: String,
    pub name: String,
    pub size: String,
    #[serde(rename = "type")]
    pub monster_type: String,
    pub subtype: Option<String>,
    pub alignment: String,
    pub armor_class: i32,
    pub hit_points: i32,
    pub hit_dice: String,
    pub speed: Speed,
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub proficiencies: Vec<MonsterProficiency>,
    pub damage_vulnerabilities: Vec<String>,
    pub damage_resistances: Vec<String>,
    pub damage_immunities: Vec<String>,
    pub condition_immunities: Vec<Condition>,
    pub senses: Senses,
    pub languages: String,
    pub challenge_rating: f32,
    pub xp: i32,
    pub special_abilities: Option<Vec<SpecialAbility>>,
    pub actions: Option<Vec<Action>>,
    pub desc: Option<Vec<String>>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Action {
    name: String,
    desc: String,
    attack_bonus: Option<i32>,
    dc: Option<Dc>,
    damage: Option<Vec<Damage>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SpecialAbility {
    name: String,
    desc: String,
    dc: Option<Dc>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct MonsterProficiency {
    value: i32,
    proficiency: Proficiency,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Speed {
    walk: Option<String>,
    swim: Option<String>,
    fly: Option<String>,
    burrow: Option<String>,
    climb: Option<String>,
    hover: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Senses {
    passive_perception: i32,
}
