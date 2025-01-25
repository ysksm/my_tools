use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::fs;

#[derive(Debug, Deserialize)]
struct Config {
    api: ApiConfig,
}

#[derive(Debug, Deserialize)]
struct ApiConfig {
    jira_base_url: String,
    jira_username: String,
    jira_api_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Project {
    id: String,
    key: String,
    name: String,
    #[serde(rename = "projectTypeKey")]
    project_type_key: String,
}

async fn request(url: &str, headers: HeaderMap) -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .headers(headers)
        .send()
        .await?;

    println!("Status: {}", response.status());
    
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    let body = response.text().await?;
    Ok(body)
}

async fn create_headers(username: &str, api_token: &str) -> Result<HeaderMap, Box<dyn Error>> {
    let mut headers = HeaderMap::new();
    
    // Basic認証ヘッダーの作成
    let auth = format!("{}:{}", username, api_token);
    let encoded_auth = STANDARD.encode(auth.as_bytes());
    let auth_header = format!("Basic {}", encoded_auth);
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&auth_header)?
    );

    // Content-Typeヘッダーの設定
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/json")
    );

    // Accept ヘッダーの追加
    headers.insert(
        reqwest::header::ACCEPT,
        HeaderValue::from_static("application/json")
    );
    Ok(headers)
}

async fn request_api(headers: HeaderMap, url: &str) {
    println!("Requesting projects from: {}", url);
    let response = request(url, headers).await;
    match response {
        Ok(_body) => {
            // println!("Response: {}", body);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 設定ファイルを読み込む
    let config_data = fs::read_to_string("config.toml").await?;
    let config: Config = toml::from_str(&config_data)?;
    
    // ヘッダーを作成
    let headers = create_headers(&config.api.jira_username, &config.api.jira_api_token).await?;

    // JIRAのプロジェクト一覧を取得
    let projects_url = format!("{}/rest/api/3/project", config.api.jira_base_url);
    request_api(headers, &projects_url).await;


    Ok(())
}
