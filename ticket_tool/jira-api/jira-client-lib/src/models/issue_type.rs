use serde::{Deserialize, Serialize};
use super::Scope;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IssueTypeDetails {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub icon_url: Option<String>,
    #[serde(rename = "self")]
    pub self_url: String,
    pub subtask: bool,
    pub avatar_id: Option<i64>,
    pub entity_id: Option<String>,
    pub hierarchy_level: Option<i32>,
    pub scope: Option<Scope>,
}