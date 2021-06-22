use serde::{Deserialize, Serialize};
use crate::documents::RuleSection;

#[derive(Serialize, Deserialize, Clone)]
pub struct Rule {
    pub name: String,
    pub index: String,
    pub desc: String,
    pub subsections: Vec<RuleSection>,
    pub url: String,
}
