use crate::{client::JiraClient, error::Result, models::FieldDetails};

impl JiraClient {
    /// Get all fields
    pub async fn get_fields(&self) -> Result<Vec<FieldDetails>> {
        self.get("/rest/api/3/field").await
    }
}