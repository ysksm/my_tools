use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialInfo {
    pub end_point: String,
    pub user_name: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectInfo {
    pub project_key: String,
    pub project_name: String,
    pub is_sync: bool,
    pub where_condition: String,
    pub order_by: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConnectSetting {
    pub credential_info: CredentialInfo,
    pub project_infos: Vec<ProjectInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProjectSyncData {
    pub project_key: String,
    pub project_name: String,
    pub last_updated: DateTime<Utc>,
    pub last_updated_issue_keys: Vec<String>,
}

impl ConnectSetting {
    pub fn create_default() -> Self {
        Self {
            credential_info: CredentialInfo {
                end_point: String::new(),
                user_name: String::new(),
                password: String::new(),
            },
            project_infos: Vec::new(),
        }
    }
}

impl ProjectSyncData {
    pub fn create_default(project: &ProjectInfo) -> Self {
        Self {
            project_key: project.project_key.clone(),
            project_name: project.project_name.clone(),
            last_updated: Utc::now() - chrono::Duration::days(3000),
            last_updated_issue_keys: Vec::new(),
        }
    }
}
