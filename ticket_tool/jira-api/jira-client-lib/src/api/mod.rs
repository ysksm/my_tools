pub mod field;
pub mod issue_type;
pub mod priority;
pub mod project;
pub mod search;
pub mod status;
pub mod user;

use crate::{client::JiraClient, error::{Error, Result}};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

impl JiraClient {
    pub(crate) async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = self.build_url(path)?;
        let response = self.client
            .get(url)
            .headers(self.build_headers())
            .send()
            .await?;
        
        handle_response(response).await
    }
    
    pub(crate) async fn post<B: serde::Serialize, T: DeserializeOwned>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path)?;
        let response = self.client
            .post(url)
            .headers(self.build_headers())
            .json(body)
            .send()
            .await?;
        
        handle_response(response).await
    }
}

async fn handle_response<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    let status = response.status();
    
    match status {
        StatusCode::OK | StatusCode::CREATED => {
            response.json::<T>().await.map_err(Into::into)
        }
        StatusCode::UNAUTHORIZED => {
            Err(Error::AuthenticationFailed("Invalid credentials".to_string()))
        }
        StatusCode::NOT_FOUND => {
            Err(Error::NotFound("Resource not found".to_string()))
        }
        StatusCode::TOO_MANY_REQUESTS => {
            Err(Error::RateLimitExceeded)
        }
        _ => {
            let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
            })
        }
    }
}