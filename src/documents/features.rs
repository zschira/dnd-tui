use serde::{Deserialize, Serialize};
use crate::documents::Class;

#[derive(Serialize, Deserialize, Clone)]
pub struct Feature {
    pub index: String,
    pub name: String,
    pub class: Option<Class>,
    pub desc: Option<Vec<String>>,
    pub choice: Option<FeatureMin>,
    pub url: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct FeatureMin {
    pub index: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}
