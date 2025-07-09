use serde::{Deserialize, Serialize};
use super::AvatarUrls;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub account_id: String,
    pub account_type: Option<String>,
    pub display_name: String,
    pub email_address: Option<String>,
    pub active: bool,
    pub avatar_urls: Option<AvatarUrls>,
    #[serde(rename = "self")]
    pub self_url: String,
    pub time_zone: Option<String>,
    pub locale: Option<String>,
    pub groups: Option<UserGroups>,
    pub application_roles: Option<UserApplicationRoles>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroups {
    pub size: i32,
    pub items: Vec<GroupName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupName {
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserApplicationRoles {
    pub size: i32,
    pub items: Vec<ApplicationRole>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationRole {
    pub key: String,
    pub name: String,
}