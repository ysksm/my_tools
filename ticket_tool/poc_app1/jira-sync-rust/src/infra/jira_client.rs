use reqwest::{Client, header};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

use crate::error::{JiraSyncError, Result};
use crate::models::{
    config::CredentialInfo,
    jira::{Project, Field, SearchRequest, SearchResponse},
};

pub struct JiraClient {
    client: Client,
    credentials: CredentialInfo,
}

impl JiraClient {
    pub fn new(credentials: CredentialInfo) -> Result<Self> {
        let mut headers = header::HeaderMap::new();
        let auth = format!("{}:{}", credentials.user_name, credentials.password);
        let auth_header = format!("Basic {}", BASE64.encode(auth));
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&auth_header)
                .map_err(|e| JiraSyncError::Config(format!("Invalid auth header: {}", e)))?,
        );

        let client = Client::builder()
            .default_headers(headers)
            .build()
            .map_err(|e| JiraSyncError::Config(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            client,
            credentials,
        })
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>> {
        let url = format!("{}/rest/api/2/project", self.credentials.end_point);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| JiraSyncError::JiraApi(format!("Failed to get projects: {}", e)))?;

        if !response.status().is_success() {
            return Err(JiraSyncError::JiraApi(format!(
                "Failed to get projects: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| JiraSyncError::JiraApi(format!("Failed to parse projects response: {}", e)))
    }

    pub async fn get_fields(&self) -> Result<Vec<Field>> {
        let url = format!("{}/rest/api/2/field", self.credentials.end_point);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| JiraSyncError::JiraApi(format!("Failed to get fields: {}", e)))?;

        if !response.status().is_success() {
            return Err(JiraSyncError::JiraApi(format!(
                "Failed to get fields: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| JiraSyncError::JiraApi(format!("Failed to parse fields response: {}", e)))
    }

    pub async fn search_issues(&self, request: SearchRequest) -> Result<SearchResponse> {
        let url = format!("{}/rest/api/2/search", self.credentials.end_point);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| JiraSyncError::JiraApi(format!("Failed to search issues: {}", e)))?;

        if !response.status().is_success() {
            return Err(JiraSyncError::JiraApi(format!(
                "Failed to search issues: {}",
                response.status()
            )));
        }

        response
            .json()
            .await
            .map_err(|e| JiraSyncError::JiraApi(format!("Failed to parse search response: {}", e)))
    }
}
