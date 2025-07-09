use crate::{client::JiraClient, error::Result, models::StatusCategory};

impl JiraClient {
    /// Get all status categories
    pub async fn get_status_categories(&self) -> Result<Vec<StatusCategory>> {
        self.get("/rest/api/3/statuscategory").await
    }
}