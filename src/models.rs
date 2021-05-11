use std::convert::TryFrom;

pub enum Damage {
    Acid = 0,
    Bludgeoning = 1,
    Cold = 2,
    Fire = 3,
    Force = 4,
    Lightning = 5,
    Necrotic = 6,
    Piercing = 7,
    Poison = 8,
    Psychic = 9,
    Radiant = 10,
    Slashing = 11,
    Thunder = 12,
}

impl TryFrom<String> for Damage {
    type Error = &'static str;

    fn try_from(damage_type: String) -> Result<Self, Self::Error> {
        match damage_type.as_str() {
            "acid" => Ok(Damage::Acid),
            "bludgeoning" => Ok(Damage::Bludgeoning),
            "cold" => Ok(Damage::Cold),
            "fire" => Ok(Damage::Fire),
            "force" => Ok(Damage::Force),
            "lightning" => Ok(Damage::Lightning),
            "necrotic" => Ok(Damage::Necrotic),
            "piercing" => Ok(Damage::Piercing),
            "poison" => Ok(Damage::Poison),
            "psychic" => Ok(Damage::Psychic),
            "radiant" => Ok(Damage::Radiant),
            "slashing" => Ok(Damage::Slashing),
            "thunder" => Ok(Damage::Thunder),
            _ => Err("Invalid damage type"),
        }
    }
}

pub enum School {
    Abjuration = 0,
    Conjuration = 1,
    Divination = 2,
    Enchantment = 3,
    Evocation = 4,
    Illusion = 5,
    Necromancy = 6,
    Transmutation = 7,
}

impl TryFrom<String> for School {
    type Error = &'static str;

    fn try_from(school: String) -> Result<Self, Self::Error> {
        match school.as_str() {
            "abjuration" => Ok(School::Abjuration),
            "conjuration" => Ok(School::Conjuration),
            "divination" => Ok(School::Divination),
            "enchantment" => Ok(School::Enchantment),
            "evocation" => Ok(School::Evocation),
            "illusion" => Ok(School::Illusion),
            "necromancy" => Ok(School::Necromancy),
            "transmutation" => Ok(School::Transmutation),
            _ => Err("Invalid school"),
        }
    }
}

pub enum Class {
    Barbarian = 0,
    Bard = 1,
    Cleric = 2,
    Druid = 3,
    Fighter = 4,
    Monk = 5,
    Paladin = 6,
    Ranger = 7,
    Rogue = 8,
    Sorcerer = 9,
    Warlock = 10,
    Wizard = 11,
}

impl TryFrom<String> for Class {
    type Error = String;

    fn try_from(class: String) -> Result<Self, Self::Error> {
        match class.as_str() {
            "barbarian" => Ok(Class::Barbarian),
            "bard" => Ok(Class::Bard),
            "cleric" => Ok(Class::Cleric),
            "druid" => Ok(Class::Druid),
            "fighter" => Ok(Class::Fighter),
            "monk" => Ok(Class::Monk),
            "paladin" => Ok(Class::Paladin),
            "ranger" => Ok(Class::Ranger),
            "rogue" => Ok(Class::Rogue),
            "sorcerer" => Ok(Class::Sorcerer),
            "warlock" => Ok(Class::Warlock),
            "wizard" => Ok(Class::Wizard),
            _ => Err(format!("Invalid class {}", class)),
        }
    }
}
