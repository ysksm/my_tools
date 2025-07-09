use jira_client_lib::{Auth, JiraClient, JiraConfig, SearchRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get configuration from environment variables
    let base_url = env::var("JIRA_BASE_URL")
        .unwrap_or_else(|_| "https://your-domain.atlassian.net".to_string());
    let username = env::var("JIRA_USERNAME")
        .unwrap_or_else(|_| "your-email@example.com".to_string());
    let api_token = env::var("JIRA_API_TOKEN")
        .unwrap_or_else(|_| "your-api-token".to_string());
    
    // Create JIRA client configuration
    let config = JiraConfig::new(
        base_url,
        Auth::Basic { username, api_token },
    )?;
    
    // Create JIRA client
    let client = JiraClient::new(config)?;
    
    // Example 1: Get all projects
    println!("Fetching all projects...");
    let projects = client.get_projects().await?;
    for project in &projects {
        println!("Project: {} ({})", project.name, project.key);
    }
    
    // Example 2: Get all issue types
    println!("\nFetching issue types...");
    let issue_types = client.get_issue_types().await?;
    for issue_type in &issue_types {
        println!("Issue Type: {} (subtask: {})", issue_type.name, issue_type.subtask);
    }
    
    // Example 3: Get all priorities
    println!("\nFetching priorities...");
    let priorities = client.get_priorities().await?;
    for priority in &priorities {
        println!("Priority: {} - {}", priority.name, priority.description.as_deref().unwrap_or(""));
    }
    
    // Example 4: Search for issues using JQL (GET method)
    println!("\nSearching for issues...");
    let jql = "project = TEST ORDER BY created DESC";
    let search_results = client.search_issues_get(
        jql,
        Some(0),      // start at
        Some(10),     // max results
        Some(&["summary", "status", "assignee"]), // fields to include
        None,         // expand options
    ).await?;
    
    println!("Found {} issues (showing {} of {})", 
        search_results.total, 
        search_results.issues.len(), 
        search_results.total
    );
    
    for issue in &search_results.issues {
        println!("Issue: {} - {}", issue.key, issue.id);
        if let Some(summary) = issue.fields.get("summary") {
            println!("  Summary: {}", summary);
        }
    }
    
    // Example 5: Search for issues using JQL (POST method)
    println!("\nSearching for issues (POST method)...");
    let search_request = SearchRequest {
        jql: "project = TEST AND status = 'In Progress'".to_string(),
        start_at: Some(0),
        max_results: Some(5),
        fields: Some(vec!["summary".to_string(), "status".to_string()]),
        expand: None,
        validate_query: None,
    };
    
    let post_results = client.search_issues_post(&search_request).await?;
    println!("Found {} issues matching criteria", post_results.total);
    
    // Example 6: Search for users
    println!("\nSearching for users...");
    let users = client.search_users(
        Some("admin"),  // query string
        Some(0),        // start at
        Some(10),       // max results
    ).await?;
    
    for user in &users {
        println!("User: {} ({})", user.display_name, user.account_id);
    }
    
    Ok(())
}