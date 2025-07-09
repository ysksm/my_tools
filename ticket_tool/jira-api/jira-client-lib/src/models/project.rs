use serde::{Deserialize, Serialize};
use super::{AvatarUrls, User};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub project_type_key: Option<String>,
    pub style: Option<String>,
    #[serde(rename = "self")]
    pub self_url: String,
    pub avatar_urls: Option<AvatarUrls>,
    pub project_category: Option<ProjectCategory>,
    pub lead: Option<User>,
    pub archived: Option<bool>,
    pub deleted: Option<bool>,
    pub simplified: Option<bool>,
    pub insight: Option<ProjectInsight>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInsight {
    pub last_issue_update_time: Option<String>,
    pub total_issue_count: Option<i64>,
}