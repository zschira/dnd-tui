use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Condition {
    pub index: String,
    pub name: String,
    pub desc: Option<Vec<String>>,
    pub url: String,
}
