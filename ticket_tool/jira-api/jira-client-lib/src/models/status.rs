use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusCategory {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub color_name: String,
    #[serde(rename = "self")]
    pub self_url: String,
}