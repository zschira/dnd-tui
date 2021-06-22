use serde::{Deserialize, Serialize};
use crate::documents::EquipmentCategory;

#[derive(Serialize, Deserialize, Clone)]
pub struct MagicItem {
    pub index: String,
    pub name: String,
    pub equipment_category: EquipmentCategory,
    pub desc: Vec<String>,
    pub url: String,
}
