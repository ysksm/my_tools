use std::{collections::HashMap, error::Error};
use base64::Engine;
use base64::engine::general_purpose::STANDARD;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use tokio::fs;
use duckdb::{params, Connection, Result}
;

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    api: ApiConfig,
    setting: SettingConfig,
}

#[derive(Debug, Deserialize, Serialize)]
struct ApiConfig {
    jira_base_url: String,
    jira_username: String,
    jira_api_token: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct SettingConfig {
    output_dir: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct ProjectInfo {
    key: String,
    id: String,
    is_sync: bool,
    where_condition: String,
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

#[derive(Debug, Deserialize)]
struct Response2 {
    // issues: Vec<serde_json::Value>,
    // names: Vec<serde_json::Value>,
    // schema: Vec<serde_json::Value>,
    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,
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

async fn init_db(config: &Config, headers: &HeaderMap, conn: &mut Connection) -> Result<(), Box<dyn Error>> {

    // プロジェクト一覧取得
    let projects_search_url = format!("{}/rest/api/3/project/search?expand=description,projectKeys,lead,issueTypes,url,insight", config.api.jira_base_url);
    let projects_ourput_dir = format!("{}/{}", config.setting.output_dir, PROJECT_JSON_FILE_PATH);
    request_api(headers.clone(), &projects_search_url, &projects_ourput_dir).await;

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

    // フィールド一覧取得
    let field_paginated_url = format!("{}/rest/api/3/field/search?orderBy=name", config.api.jira_base_url);
    let fields_output_dir = format!("{}/{}", config.setting.output_dir, FIELDS_JSON_FILE_PATH);
    request_api(headers.clone(), &field_paginated_url, &fields_output_dir).await;

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
    Ok(())
}

async fn request_post(url: &str, headers: HeaderMap, request_body: &HashMap<String, serde_json::Value> ) -> Result<String, Box<dyn Error>> {
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

async fn create_issue_request_body(jql: &str, fields: Vec<String>, next_page_token: Option<String>) -> HashMap<String, serde_json::Value> {

   // Issue検索
   let expand = "renderedFields,names,schema,transitions,operations,editmeta,changelog,versionedRepresentations";
//    let fields = vec!["summary", "description", "status"];  // 必要なフィールドを追加
   let max_results = 5;

    let mut request_body = HashMap::new();
    request_body.insert("expand".to_string(), serde_json::Value::String(expand.to_string()));
    request_body.insert("fields".to_string(), serde_json::Value::Array(fields.into_iter().map(|s| serde_json::Value::String(s.to_string())).collect()));
    request_body.insert("jql".to_string(), serde_json::Value::String(jql.to_string()));
    request_body.insert("maxResults".to_string(), serde_json::Value::Number(max_results.into()));
    if next_page_token.is_some() {
        request_body.insert("nextPageToken".to_string(), serde_json::Value::String(next_page_token.unwrap().to_string()));
    }
    request_body
}

async fn sync_issues(config: &Config,headers: HeaderMap, conn: &mut Connection) -> Result<(), Box<dyn Error>> {
    let url = format!("{}/rest/api/3/search/jql", config.api.jira_base_url);
    let dir_path = format!("{}/issues", config.setting.output_dir);
    if !std::path::Path::new(&dir_path).exists() {
        fs::create_dir_all(&dir_path).await.unwrap();
    }

    // DBからフィールド一覧取得
    let mut fields = Vec::new();
    let mut stmt = conn.prepare("SELECT id FROM fields")?;
    let mut rows = stmt.query(params![])?;
    while let Some(row) = rows.next()? {
        let field_name: String = row.get("id")?;
        fields.push(field_name);
    }
    
    let mut is_last = false;
    let mut page_count = 0;
    let mut next_page_token = None;
    let jql = "project=todo order by updated ASC";

    while !is_last {
        let request_body = create_issue_request_body(&jql, fields.clone(), next_page_token.clone()).await;
        let response = request_post(&url, headers.clone(), &request_body).await;
        match response {
            Ok(body) => {
                let file_path = format!("{}/{}.json", dir_path, page_count);
                fs::write(&file_path, &body).await.unwrap();
                let response: Response2 = serde_json::from_str(&body).unwrap();
                if let Some(next_page_token_text) = response.next_page_token {
                    next_page_token = Some(next_page_token_text);
                } else {
                    next_page_token = None;
                    is_last = true;
                }
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
        page_count = page_count + 1;
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    // 設定ファイルを読み込む
    let config_data = fs::read_to_string("config.toml").await?;
    let config: Config = toml::from_str(&config_data)?;

    // DBファイルの準備
    let db_path = "jira.db";
    // dbファイルが存在したら削除する
    let db_path = format!("{}/{}", &config.setting.output_dir, db_path);
    if std::path::Path::new(&db_path).exists() {
        fs::remove_file(&db_path).await?;
        println!("Removed existing db file");
    }
    let mut conn = Connection::open(db_path)?;
    // ヘッダーを作成
    let headers = create_headers(&config.api.jira_username, &config.api.jira_api_token).await?;
    init_db(&config, &headers, &mut conn).await?;

    // dbからプロジェクトの一覧を取得
    let project_info_file_path = format!("{}/projects_info.json", config.setting.output_dir);
    
    let mut stmt = conn.prepare("SELECT id, key FROM projects")?;
    let mut rows = stmt.query(params![])?;
    let mut project_infos: Vec<ProjectInfo> = Vec::new();

    while let Some(row) = rows.next()? {
        let id: String = row.get("id")?;
        let key: String = row.get("key")?;
        let project_info = ProjectInfo {
            key: key.clone(),
            id: id,
            is_sync: false,
            where_condition: format!("project = '{}'", key.clone()).to_string(),
        };
        project_infos.push(project_info);
    }
    // output_dirにprojects_info.jsonを作成
    fs::write(&project_info_file_path, serde_json::to_string(&project_infos).unwrap()).await.unwrap();



    sync_issues(&config, headers, &mut conn).await?;

    conn.close().unwrap();
    println!("Done");

    Ok(())
}
