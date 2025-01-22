use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub key: String,
    pub name: String,
    #[serde(rename = "projectTypeKey")]
    pub project_type_key: String,
    pub simplified: bool,
    pub style: String,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldSchema {
    pub r#type: String,
    pub items: Option<String>,
    pub system: Option<String>,
    pub custom: Option<String>,
    #[serde(rename = "customId")]
    pub custom_id: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Field {
    pub id: String,
    pub key: Option<String>,
    pub name: String,
    pub custom: bool,
    pub orderable: bool,
    pub navigable: bool,
    pub searchable: bool,
    pub schema: Option<FieldSchema>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchRequest {
    pub jql: String,
    #[serde(rename = "startAt")]
    pub start_at: i32,
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    pub fields: Vec<String>,
    pub expand: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub key: String,
    pub expand: Option<String>,
    #[serde(rename = "self")]
    pub self_link: String,
    pub fields: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub expand: Option<String>,
    #[serde(rename = "startAt")]
    pub start_at: i32,
    #[serde(rename = "maxResults")]
    pub max_results: i32,
    pub total: i32,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "emailAddress")]
    pub email_address: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub active: bool,
    pub locale: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Priority {
    pub id: String,
    pub name: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub id: String,
    pub name: String,
    #[serde(rename = "statusCategory")]
    pub status_category: StatusCategory,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusCategory {
    pub id: i32,
    pub key: String,
    #[serde(rename = "colorName")]
    pub color_name: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IssueType {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "iconUrl")]
    pub icon_url: String,
    pub subtask: bool,
}
