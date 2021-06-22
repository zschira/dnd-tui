use serde::{Deserialize, Serialize};
use crate::documents::Equipment;

#[derive(Serialize, Deserialize, Clone)]
pub struct EquipmentCategory {
    pub index: String,
    pub name: String,
    pub equipment: Option<Vec<Equipment>>,
    pub url: String,
}
