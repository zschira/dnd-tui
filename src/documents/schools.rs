use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct School {
    pub index: String,
    pub name: String,
    pub desc: Option<String>,
    pub url: String,
}
