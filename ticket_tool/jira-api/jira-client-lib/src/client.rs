use crate::error::Result;
use reqwest::{Client, header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE}};
use url::Url;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Auth {
    Basic { username: String, api_token: String },
    Bearer { token: String },
}

#[derive(Debug, Clone)]
pub struct JiraConfig {
    pub base_url: String,
    pub auth: Auth,
}

impl JiraConfig {
    pub fn new(base_url: impl Into<String>, auth: Auth) -> Result<Self> {
        let base_url = base_url.into();
        
        // Validate URL
        let _ = Url::parse(&base_url)
            .map_err(|_| crate::error::Error::InvalidConfiguration("Invalid base URL".to_string()))?;
        
        Ok(Self {
            base_url,
            auth,
        })
    }
}

#[derive(Debug, Clone)]
pub struct JiraClient {
    pub(crate) client: Client,
    pub(crate) config: Arc<JiraConfig>,
}

impl JiraClient {
    pub fn new(config: JiraConfig) -> Result<Self> {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        
        Ok(Self {
            client,
            config: Arc::new(config),
        })
    }
    
    pub(crate) fn build_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        
        match &self.config.auth {
            Auth::Basic { username, api_token } => {
                let credentials = base64::Engine::encode(
                    &base64::engine::general_purpose::STANDARD,
                    format!("{username}:{api_token}")
                );
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Basic {credentials}"))
                        .expect("Invalid auth header")
                );
            }
            Auth::Bearer { token } => {
                headers.insert(
                    AUTHORIZATION,
                    HeaderValue::from_str(&format!("Bearer {token}"))
                        .expect("Invalid auth header")
                );
            }
        }
        
        headers
    }
    
    pub(crate) fn build_url(&self, path: &str) -> Result<Url> {
        let base_url = Url::parse(&self.config.base_url)
            .map_err(|_| crate::error::Error::InvalidConfiguration("Invalid base URL".to_string()))?;
        
        base_url.join(path)
            .map_err(|_| crate::error::Error::InvalidConfiguration(format!("Invalid path: {path}")))
    }
}