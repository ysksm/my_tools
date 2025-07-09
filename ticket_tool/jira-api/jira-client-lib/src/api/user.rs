use crate::{client::JiraClient, error::Result, models::User};

impl JiraClient {
    /// Search for users
    pub async fn search_users(
        &self,
        query: Option<&str>,
        start_at: Option<i32>,
        max_results: Option<i32>,
    ) -> Result<Vec<User>> {
        let mut url = self.build_url("/rest/api/3/users/search")?;
        
        {
            let mut pairs = url.query_pairs_mut();
            if let Some(q) = query {
                pairs.append_pair("query", q);
            }
            if let Some(start) = start_at {
                pairs.append_pair("startAt", &start.to_string());
            }
            if let Some(max) = max_results {
                pairs.append_pair("maxResults", &max.to_string());
            }
        }
        
        let response = self.client
            .get(url)
            .headers(self.build_headers())
            .send()
            .await?;
        
        super::handle_response(response).await
    }
}