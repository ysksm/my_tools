use std::{collections::HashMap, error::Error};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;
use tokio::fs;
use duckdb::{params, Connection, Result}
;

#[derive(Debug, Deserialize)]
struct Config {
    api: ApiConfig,
    setting: SettingConfig,
}

#[derive(Debug, Deserialize)]
struct ApiConfig {
    jira_base_url: String,
    jira_username: String,
    jira_api_token: String,
}

#[derive(Debug, Deserialize)]
struct SettingConfig {
    output_dir: String,
}

#[derive(Debug, Deserialize)]
struct Response {
    #[serde(rename = "maxResults")]
    max_results: i32,

    #[serde(rename = "startAt")]
    start_at: i32,

    total: i32,
    
    #[serde(rename = "isLast")]
    is_last: bool,
    values: Vec<serde_json::Value>,
}

const PROJECT_JSON_FILE_PATH: &str = "projects_search";
const FIELDS_JSON_FILE_PATH: &str = "fields_search";

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

async fn request_get(url: &str, headers: HeaderMap) -> Result<String, Box<dyn Error>> {
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

async fn request_post(url: &str, headers: HeaderMap, request_body: &HashMap<String, String> ) -> Result<String, Box<dyn Error>> {
    println!("Requesting: {}", url);

    let client = reqwest::Client::new();
    let response = client
        .post(url)
        .headers(headers)
        .json(request_body)
        .send()
        .await?;

    println!("Status: {}", response.status());
    
    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    let body = response.text().await?;

    println!("Response: {}", body);

    Ok(body)
}

async fn request_api(headers: HeaderMap, url: &str, dir_path: &str) {

    let mut is_last = false;
    let mut page_count = 0;
    let start_at = 0;

    if !std::path::Path::new(&dir_path).exists() {
        fs::create_dir_all(&dir_path).await.unwrap();
    }

    
    // ページネーションを考慮してリクエストを送信
    while !is_last {
        let request_url = format!("{}&startAt={}", url, page_count * 50);
        println!("Requesting projects from: {}", request_url);
        let response = request_get(&request_url, headers.clone()).await;
        match response {
            Ok(body) => {
                
                let temp_file_path = format!("{}/{}.json", dir_path, page_count);
                // bodyをファイルに書き込む
                match fs::write(&temp_file_path, &body).await {
                    Ok(_) => println!("Successfully wrote response to {}", temp_file_path),
                    Err(e) => println!("Error writing to file: {}", e),
                }
                let response: Response = serde_json::from_str(&body).unwrap();
                let file_path = format!("{}/{}.json", dir_path, page_count);
                fs::rename(&temp_file_path, &file_path).await.unwrap();
                is_last = response.is_last;
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        page_count = page_count + 1;
    }

}

async fn init_db(config: &Config, headers: &HeaderMap) -> Result<(), Box<dyn Error>> {

    // プロジェクト一覧取得
    let projects_search_url = format!("{}/rest/api/3/project/search?expand=description,projectKeys,lead,issueTypes,url,insight", config.api.jira_base_url);
    let projects_ourput_dir = format!("{}/{}", config.setting.output_dir, PROJECT_JSON_FILE_PATH);
    request_api(headers.clone(), &projects_search_url, &projects_ourput_dir).await;

    // フィールド一覧取得
    let filed_painated_url = format!("{}/rest/api/3/field/search?orderBy=name", config.api.jira_base_url);
    let fields_output_dir = format!("{}/{}", config.setting.output_dir, FIELDS_JSON_FILE_PATH);
    request_api(headers.clone(), &filed_painated_url, &fields_output_dir).await;

    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // 設定ファイルを読み込む
    let config_data = fs::read_to_string("config.toml").await?;
    let config: Config = toml::from_str(&config_data)?;

    let db_path = "jira.db";


    // ヘッダーを作成
    let headers = create_headers(&config.api.jira_username, &config.api.jira_api_token).await?;

    init_db(&config, &headers).await?;


    // Issue検索
    let jql = "project=todo";
    let max_results = 50;
    let fields = ["summary"];
    let fiedlds_str = format!("[\"{}\"]", fields.join("\",\""));
    println!("Fields: {}", fiedlds_str);
    let expand = "changelog,names";
    let issue_output_dir = format!("{}/issues", config.setting.output_dir);

    let issue_url = format!("{}/rest/api/3/search/jql", config.api.jira_base_url);
    let mut request_body = HashMap::new();
    request_body.insert("jql".to_string(), jql.to_string());
    request_body.insert("maxResults".to_string(), max_results.to_string());
    // request_body.insert("fields".to_string(), fiedlds);
    // request_body.insert("expand".to_string(), expand.to_string());

    let response = request_post(&issue_url, headers.clone(), &request_body).await;
    match response {
        Ok(body) => {
            if !std::path::Path::new(&issue_output_dir).exists() {
                fs::create_dir_all(&issue_output_dir).await.unwrap();
            }
            let file_path = format!("{}/issues.json", issue_output_dir);
            fs::write(&file_path, &body).await.unwrap();
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    // dbファイルが存在したら削除する
    let db_path = format!("{}/{}", &config.setting.output_dir, db_path);
    if std::path::Path::new(&db_path).exists() {
        fs::remove_file(&db_path).await?;
        println!("Removed existing db file");
    }
    let conn = Connection::open(db_path)?;

    // プロジェクト一覧テーブル作成
    let create_sql = format!("CREATE TABLE projects AS 
        SELECT 
            unnest(values)->>'id' as id,
            unnest(values)->>'key' as key,
            unnest(values)->>'name' as name,
            unnest(values)->>'projectTypeKey' as projectTypeKey,
            unnest(values)->>'simplified' as simplified,
            unnest(values)->>'style' as style,
            unnest(values)->>'isPrivate' as isPrivate,
            unnest(values)->>'uuid' as uuid,
            unnest(values)->>'insight' as insight,
            unnest(values)->>'lead' as lead,
            unnest(values)->>'issueTypes' as issueTypes,
            unnest(values)->>'avatarUrls' as avatarUrls,
            unnest(values)->>'description' as description
        FROM read_json_auto('./{}/{}/*.json')", &config.setting.output_dir, PROJECT_JSON_FILE_PATH);
    conn.execute(&create_sql, params![])?;

    // フィールド一覧テーブル作成
    let create_sql = format!("CREATE TABLE fields AS 
        SELECT 
            unnest(values)->>'id' as id,
            unnest(values)->>'name' as name,
            unnest(values)->>'custom' as custom,
            unnest(values)->>'orderable' as orderable,
            unnest(values)->>'navigable' as navigable,
            unnest(values)->>'searchable' as searchable,
            unnest(values)->>'clauseNames' as clauseNames,
            unnest(values)->>'schema' as schema,
            unnest(values)->>'schema'->>'type' as schema_type,
            unnest(values)->>'schema'->>'system' as schema_system,
            unnest(values)->>'schema'->>'items' as schema_items,
            unnest(values)->>'schema'->>'custom' as schema_custom,
            unnest(values)->>'schema'->>'customId' as schema_customId,
        FROM read_json_auto('./{}/{}/*.json')", &config.setting.output_dir, FIELDS_JSON_FILE_PATH);
    conn.execute(&create_sql, params![])?;

    conn.close().unwrap();
    println!("Done");

    Ok(())
}
