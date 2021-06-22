use serde::{Deserialize, Serialize};
use crate::documents::Class;

#[derive(Serialize, Deserialize, Clone)]
pub struct Subclass {
    pub index: String,
    pub class: Option<Class>,
    pub name: String,
    pub subclass_flavor: Option<String>,
    pub desc: Option<Vec<String>>,
    pub subclass_levels: Option<String>,
    pub url: String,
}
