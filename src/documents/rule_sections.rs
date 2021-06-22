use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RuleSection {
    pub name: String,
    pub index: String,
    pub desc: Option<String>,
    pub url: String,
}
