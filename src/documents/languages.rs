use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Language {
    pub index: String,
    pub name: String,
    pub desc: Option<String>,
    #[serde(rename = "type")]
    pub lang_type: Option<String>,
    pub typical_speakers: Option<Vec<String>>,
    pub script: Option<String>,
    pub url: String,
}
