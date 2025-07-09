# jira-client-lib

A Rust library for interacting with the JIRA REST API v3.

## Features

This library implements the following JIRA API endpoints:

- **Projects**: Get all projects or a specific project
- **Fields**: Get all field definitions
- **Issue Types**: Get all issue types
- **Priorities**: Get all priorities
- **Status Categories**: Get all status categories
- **User Search**: Search for users
- **Issue Search**: Search for issues using JQL (both GET and POST methods)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
jira-client-lib = "0.1.0"
```

## Usage

### Basic Example

```rust
use jira_client_lib::{Auth, JiraClient, JiraConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create configuration
    let config = JiraConfig::new(
        "https://your-domain.atlassian.net",
        Auth::Basic {
            username: "your-email@example.com".to_string(),
            api_token: "your-api-token".to_string(),
        },
    )?;
    
    // Create client
    let client = JiraClient::new(config)?;
    
    // Get all projects
    let projects = client.get_projects().await?;
    for project in projects {
        println!("Project: {} ({})", project.name, project.key);
    }
    
    Ok(())
}
```

### Authentication

The library supports two authentication methods:

1. **Basic Authentication** (recommended for Atlassian Cloud):
```rust
Auth::Basic {
    username: "email@example.com".to_string(),
    api_token: "your-api-token".to_string(),
}
```

2. **Bearer Token Authentication**:
```rust
Auth::Bearer {
    token: "your-bearer-token".to_string(),
}
```

### Searching for Issues

#### Using GET method:
```rust
let results = client.search_issues_get(
    "project = TEST",  // JQL query
    Some(0),          // start at
    Some(50),         // max results
    Some(&["summary", "status"]), // fields to include
    None,             // expand options
).await?;
```

#### Using POST method:
```rust
use jira_client_lib::SearchRequest;

let request = SearchRequest {
    jql: "project = TEST AND status = 'In Progress'".to_string(),
    start_at: Some(0),
    max_results: Some(50),
    fields: Some(vec!["summary".to_string(), "status".to_string()]),
    expand: None,
    validate_query: None,
};

let results = client.search_issues_post(&request).await?;
```

## Environment Variables

For the example, you can set these environment variables:

- `JIRA_BASE_URL`: Your JIRA instance URL (e.g., https://your-domain.atlassian.net)
- `JIRA_USERNAME`: Your email address
- `JIRA_API_TOKEN`: Your API token (get it from https://id.atlassian.com/manage-profile/security/api-tokens)

## Running the Example

```bash
export JIRA_BASE_URL="https://your-domain.atlassian.net"
export JIRA_USERNAME="your-email@example.com"
export JIRA_API_TOKEN="your-api-token"

cargo run --example basic_usage
```

## Running Tests

```bash
cargo test
```

## License

This project is licensed under the MIT License.