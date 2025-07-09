use crate::{client::JiraClient, error::Result, models::Priority};

impl JiraClient {
    /// Get all priorities
    pub async fn get_priorities(&self) -> Result<Vec<Priority>> {
        self.get("/rest/api/3/priority").await
    }
}