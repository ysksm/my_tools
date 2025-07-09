use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Priority {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    pub status_color: Option<String>,
    pub is_default: Option<bool>,
    #[serde(rename = "self")]
    pub self_url: String,
    pub avatar_id: Option<i64>,
}