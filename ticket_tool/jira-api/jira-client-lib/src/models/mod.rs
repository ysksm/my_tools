pub mod field;
pub mod issue;
pub mod issue_type;
pub mod priority;
pub mod project;
pub mod search;
pub mod status;
pub mod user;

pub use field::*;
pub use issue::*;
pub use issue_type::*;
pub use priority::*;
pub use project::*;
pub use search::*;
pub use status::*;
pub use user::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvatarUrls {
    #[serde(rename = "16x16")]
    pub size_16: Option<String>,
    #[serde(rename = "24x24")]
    pub size_24: Option<String>,
    #[serde(rename = "32x32")]
    pub size_32: Option<String>,
    #[serde(rename = "48x48")]
    pub size_48: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    #[serde(rename = "type")]
    pub scope_type: Option<String>,
    pub project: Option<ScopeProject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeProject {
    pub id: Option<String>,
}