use anyhow::Result;
use jira_client_lib::{Auth, JiraClient, JiraConfig};
use jira_client_lib::storage::{JsonIssueStore, SqliteIssueStore, DuckDbIssueStore, IssueStore};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    let jira_url = env::var("JIRA_URL").expect("JIRA_URL must be set");
    let jira_user = env::var("JIRA_USER").expect("JIRA_USER must be set");
    let jira_api_token = env::var("JIRA_API_TOKEN").expect("JIRA_API_TOKEN must be set");

    let auth = Auth::ApiToken {
        email: jira_user,
        token: jira_api_token,
    };

    let config = JiraConfig::builder()
        .base_url(&jira_url)
        .auth(auth)
        .build()?;

    let client = JiraClient::new(config);

    let project_key = "TEST";
    
    println!("Fetching issues from project: {}", project_key);
    let search_results = client.search_issues(&format!("project = {}", project_key), 50, 0).await?;
    println!("Found {} issues", search_results.issues.len());

    if search_results.issues.is_empty() {
        println!("No issues found to demonstrate storage");
        return Ok(());
    }

    println!("\n=== JSON Storage Demo ===");
    let json_store = JsonIssueStore::new("./data/json_storage")?;
    
    for issue in &search_results.issues {
        json_store.store_issue(project_key, issue).await?;
        println!("Stored issue {} in JSON format", issue.key);
    }
    
    let retrieved_issues = json_store.get_all_issues(project_key).await?;
    println!("Retrieved {} issues from JSON storage", retrieved_issues.len());

    println!("\n=== SQLite Storage Demo ===");
    let sqlite_store = SqliteIssueStore::new("./data/jira_issues.db").await?;
    
    sqlite_store.store_issues(project_key, &search_results.issues).await?;
    println!("Stored {} issues in SQLite", search_results.issues.len());
    
    let sqlite_issues = sqlite_store.get_all_issues(project_key).await?;
    println!("Retrieved {} issues from SQLite", sqlite_issues.len());

    if let Some(first_issue) = search_results.issues.first() {
        let retrieved = sqlite_store.get_issue(project_key, &first_issue.key).await?;
        if retrieved.is_some() {
            println!("Successfully retrieved issue {} from SQLite", first_issue.key);
        }
    }

    println!("\n=== DuckDB Storage Demo ===");
    let duckdb_store = DuckDbIssueStore::new("./data/jira_issues.duckdb").await?;
    
    duckdb_store.store_issues(project_key, &search_results.issues).await?;
    println!("Stored {} issues in DuckDB", search_results.issues.len());
    
    let duckdb_issues = duckdb_store.get_all_issues(project_key).await?;
    println!("Retrieved {} issues from DuckDB", duckdb_issues.len());

    println!("\n✅ Storage demo completed successfully!");

    Ok(())
}