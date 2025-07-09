use crate::{client::JiraClient, error::Result, models::IssueTypeDetails};

impl JiraClient {
    /// Get all issue types
    pub async fn get_issue_types(&self) -> Result<Vec<IssueTypeDetails>> {
        self.get("/rest/api/3/issuetype").await
    }
}