use serde::{Deserialize, Serialize};
use super::Scope;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FieldDetails {
    pub id: String,
    pub key: Option<String>,
    pub name: String,
    pub custom: bool,
    pub orderable: bool,
    pub navigable: bool,
    pub searchable: bool,
    pub clause_names: Option<Vec<String>>,
    pub schema: Option<JsonTypeBean>,
    pub scope: Option<Scope>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonTypeBean {
    #[serde(rename = "type")]
    pub field_type: String,
    pub items: Option<String>,
    pub system: Option<String>,
    pub custom: Option<String>,
    #[serde(rename = "customId")]
    pub custom_id: Option<i64>,
}