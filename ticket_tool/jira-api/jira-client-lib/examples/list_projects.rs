use anyhow::Result;
use jira_client_lib::{Auth, JiraClient, JiraConfig};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // .envファイルから環境変数を読み込む
    dotenv::dotenv().ok();
    
    let jira_url = env::var("JIRA_URL").expect("JIRA_URL must be set");
    let jira_user = env::var("JIRA_USER").expect("JIRA_USER must be set");
    let jira_api_token = env::var("JIRA_API_TOKEN").expect("JIRA_API_TOKEN must be set");

    let auth = Auth::Basic {
        username: jira_user,
        api_token: jira_api_token,
    };

    let config = JiraConfig::new(jira_url, auth)?;
    let client = JiraClient::new(config)?;

    println!("Fetching all projects...\n");
    
    let projects = client.get_projects().await?;
    
    println!("Found {} projects:\n", projects.len());
    
    for project in projects {
        println!("Key: {:<10} Name: {}", project.key, project.name);
        if let Some(desc) = project.description {
            println!("  Description: {}", desc);
        }
        println!();
    }
    
    println!("\nYou can set JIRA_PROJECT_KEY environment variable to one of the keys above.");

    Ok(())
}