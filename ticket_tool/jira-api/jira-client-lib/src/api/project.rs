use crate::{client::JiraClient, error::Result, models::Project};

impl JiraClient {
    /// Get all projects visible to the user
    pub async fn get_projects(&self) -> Result<Vec<Project>> {
        self.get("/rest/api/3/project").await
    }
    
    /// Get a specific project by key or ID
    pub async fn get_project(&self, project_key_or_id: &str) -> Result<Project> {
        let path = format!("/rest/api/3/project/{project_key_or_id}");
        self.get(&path).await
    }
}