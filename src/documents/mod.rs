pub mod ability_scores;
pub mod alignments;
pub mod backgrounds;
pub mod conditions;
pub mod classes;
pub mod common;
pub mod damage_types;
pub mod equipment;
pub mod equipment_categories;
pub mod features;
pub mod languages;
pub mod levels;
pub mod magic_items;
pub mod monsters;
pub mod proficiencies;
pub mod races;
pub mod rule_sections;
pub mod rules;
pub mod schools;
pub mod skills;
pub mod spells;
pub mod starting_equipment;
pub mod subclasses;
pub mod subraces;
pub mod traits;
pub mod weapon_properties;

use serde::{Deserialize, Serialize};

pub use crate::documents::ability_scores::AbilityScore;
pub use crate::documents::alignments::Alignment;
pub use crate::documents::backgrounds::Background;
pub use crate::documents::conditions::Condition;
pub use crate::documents::classes::Class;
pub use crate::documents::common::*;
pub use crate::documents::damage_types::DamageType;
pub use crate::documents::equipment::Equipment;
pub use crate::documents::equipment_categories::EquipmentCategory;
pub use crate::documents::features::Feature;
pub use crate::documents::languages::Language;
pub use crate::documents::levels::Level;
pub use crate::documents::magic_items::MagicItem;
pub use crate::documents::monsters::Monster;
pub use crate::documents::proficiencies::Proficiency;
pub use crate::documents::races::Race;
pub use crate::documents::rule_sections::RuleSection;
pub use crate::documents::rules::Rule;
pub use crate::documents::schools::School;
pub use crate::documents::skills::Skill;
pub use crate::documents::spells::Spell;
pub use crate::documents::starting_equipment::StartingEquipment;
pub use crate::documents::subclasses::Subclass;
pub use crate::documents::subraces::Subrace;
pub use crate::documents::traits::Trait;
pub use crate::documents::weapon_properties::WeaponProperty;

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Document {
    AbilityScore(AbilityScore),
    Alignment(Alignment),
    Background(Background),
    Condition(Condition),
    Class(Class),
    DamageType(DamageType),
    Equipment(Equipment),
    EquipmentCategory(EquipmentCategory),
    Feature(Feature),
    Language(Language),
    Level(Level),
    MagicItem(MagicItem),
    Monster(Monster),
    Proficiency(Proficiency),
    Race(Race),
    RuleSection(RuleSection),
    Rule(Rule),
    School(School),
    Skill(Skill),
    Spell(Spell),
    Subclass(Subclass),
    Subrace(Subrace),
    StartingEquipment(StartingEquipment),
    Trait(Trait),
    WeaponProperty(WeaponProperty),
}
