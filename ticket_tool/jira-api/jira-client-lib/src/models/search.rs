use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::IssueBean;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub jql: String,
    pub start_at: Option<i32>,
    pub max_results: Option<i32>,
    pub fields: Option<Vec<String>>,
    pub expand: Option<Vec<String>>,
    pub validate_query: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResults {
    pub issues: Vec<IssueBean>,
    pub max_results: i32,
    pub start_at: i32,
    pub total: i32,
    pub expand: Option<String>,
    pub names: Option<HashMap<String, String>>,
    pub schema: Option<HashMap<String, FieldSchema>>,
    pub warning_messages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldSchema {
    #[serde(rename = "type")]
    pub field_type: String,
    pub items: Option<String>,
    pub system: Option<String>,
}