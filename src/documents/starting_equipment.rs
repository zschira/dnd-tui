use serde::{Deserialize, Serialize};
use crate::documents::{Choice, Class, EquipmentCurrent, EquipmentCategory};

#[derive(Serialize, Deserialize, Clone)]
pub struct StartingEquipment {
    pub index: String,
    pub class: Class,
    pub starting_equipment: Vec<EquipmentCurrent>,
    //pub starting_equipment_options: Vec<Choice<EquipmentCurrent>>,
    pub url: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum StartingEquipmentOption {
    EquipmentCurrent(EquipmentCurrent),
    EquipmentOption(EquipmentOption),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EquipmentOption {
    pub equipment_option: EquipmentChoice,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EquipmentChoice {
    pub choose: i32,
    #[serde(rename = "type")]
    pub choice_type: String,
    pub from: EquipmentCategoryWrapper,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EquipmentCategoryWrapper {
    pub equipment_category: EquipmentCategory,
}
