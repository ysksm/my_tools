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

### Issue Synchronization (Incremental Updates)

The library provides advanced synchronization capabilities for efficiently fetching issues with incremental updates:

```rust
use jira_client_lib::{FileSyncStateStore, SyncStateStore};

// Create a sync state store to persist synchronization state
let sync_store = FileSyncStateStore::new("./sync_states");

// Load previous sync state (if any)
let previous_state = sync_store.load("PROJECT_KEY").await?;

// Perform synchronization
let sync_result = client.sync_issues(
    "PROJECT_KEY",
    previous_state,
    50, // max results per page
).await?;

// Save the new sync state for next run
sync_store.save(&sync_result.new_sync_state).await?;

println!("Fetched {} issues", sync_result.total_fetched);
```

Features:
- **Incremental Updates**: Only fetches issues updated since the last sync
- **Handles Time Precision**: JIRA API only supports hour-level precision, the library handles this correctly
- **Deduplication**: Excludes already-fetched issues when multiple issues share the same update hour
- **Automatic Pagination**: Fetches all matching issues across multiple pages
- **State Persistence**: Saves sync state to enable incremental updates on subsequent runs

### Project Metadata

Fetch all metadata for a project in one call:

```rust
let metadata = client.get_project_metadata("PROJECT_KEY").await?;

// Access project details and configuration
println!("Project: {}", metadata.project.name);
println!("Fields: {} available", metadata.fields.len());
println!("Issue Types: {} available", metadata.issue_types.len());
println!("Priorities: {} available", metadata.priorities.len());
```

## Environment Variables

For the example, you can set these environment variables:

- `JIRA_BASE_URL`: Your JIRA instance URL (e.g., https://your-domain.atlassian.net)
- `JIRA_USERNAME`: Your email address
- `JIRA_API_TOKEN`: Your API token (get it from https://id.atlassian.com/manage-profile/security/api-tokens)
- `JIRA_PROJECT_KEY`: Project key for sync example (optional, defaults to "TEST")

## Running Examples

### Basic usage:
```bash
export JIRA_BASE_URL="https://your-domain.atlassian.net"
export JIRA_USERNAME="your-email@example.com"
export JIRA_API_TOKEN="your-api-token"

cargo run --example basic_usage
```

### Issue synchronization:
```bash
export JIRA_BASE_URL="https://your-domain.atlassian.net"
export JIRA_USERNAME="your-email@example.com"
export JIRA_API_TOKEN="your-api-token"
export JIRA_PROJECT_KEY="YOUR_PROJECT"

cargo run --example sync_issues
```

## Running Tests

```bash
cargo test
```

## License

This project is licensed under the MIT License.