use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct WeaponProperty {
    pub index: String,
    pub name: String,
    pub desc: Option<Vec<String>>,
    pub url: String,
}
