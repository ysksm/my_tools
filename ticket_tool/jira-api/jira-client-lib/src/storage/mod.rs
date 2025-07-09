use crate::error::Result;
use crate::models::issue::IssueBean;
use async_trait::async_trait;

#[async_trait]
pub trait IssueStore: Send + Sync {
    async fn store_issue(&self, project_key: &str, issue: &IssueBean) -> Result<()>;
    
    async fn store_issues(&self, project_key: &str, issues: &[IssueBean]) -> Result<()>;
    
    async fn get_issue(&self, project_key: &str, issue_key: &str) -> Result<Option<IssueBean>>;
    
    async fn get_all_issues(&self, project_key: &str) -> Result<Vec<IssueBean>>;
    
    async fn delete_issue(&self, project_key: &str, issue_key: &str) -> Result<()>;
    
    async fn clear_project(&self, project_key: &str) -> Result<()>;
}

pub mod json;
pub mod sqlite;
pub mod duckdb;

pub use json::JsonIssueStore;
pub use sqlite::SqliteIssueStore;
pub use duckdb::DuckDbIssueStore;

#[cfg(test)]
mod tests;