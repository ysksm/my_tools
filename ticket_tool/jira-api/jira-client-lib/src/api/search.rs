use crate::{client::JiraClient, error::Result, models::{SearchRequest, SearchResults}};

impl JiraClient {
    /// Search for issues using JQL (POST method)
    pub async fn search_issues_post(&self, request: &SearchRequest) -> Result<SearchResults> {
        self.post("/rest/api/3/search", request).await
    }
    
    /// Search for issues using JQL (GET method)
    pub async fn search_issues_get(
        &self,
        jql: &str,
        start_at: Option<i32>,
        max_results: Option<i32>,
        fields: Option<&[&str]>,
        expand: Option<&[&str]>,
    ) -> Result<SearchResults> {
        let mut url = self.build_url("/rest/api/3/search")?;
        
        {
            let mut pairs = url.query_pairs_mut();
            pairs.append_pair("jql", jql);
            
            if let Some(start) = start_at {
                pairs.append_pair("startAt", &start.to_string());
            }
            if let Some(max) = max_results {
                pairs.append_pair("maxResults", &max.to_string());
            }
            if let Some(fields_list) = fields {
                pairs.append_pair("fields", &fields_list.join(","));
            }
            if let Some(expand_list) = expand {
                pairs.append_pair("expand", &expand_list.join(","));
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