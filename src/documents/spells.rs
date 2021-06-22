use serde::{Deserialize, Serialize};
use crate::documents::{Class, Dc, Damage, School, Subclass};

#[derive(Serialize, Deserialize, Clone)]
pub struct Spell {
    pub index: String,
    pub name: String,
    pub desc: Vec<String>,
    pub higher_level: Option<Vec<String>>,
    pub range: Option<String>,
    pub components: Vec<String>,
    pub material: Option<String>,
    pub ritual: bool,
    pub duration: String,
    pub concentration: bool,
    pub casting_time: String,
    pub level: i32,
    pub attack_type: Option<String>,
    pub damage: Option<Damage>,
    pub dc: Option<Dc>,
    pub school: School,
    pub classes: Vec<Class>,
    pub subclasses: Vec<Subclass>,
    pub url: String,
}
