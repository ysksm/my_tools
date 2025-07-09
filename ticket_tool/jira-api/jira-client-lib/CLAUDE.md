# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust library (`jira-client-lib`) for interacting with the JIRA API v3. The project is in early development stage and needs to implement client functionality for specific JIRA REST API endpoints as defined in `spec/todo.md`.

## Key Commands

### Build and Development
```bash
# Build the project
cargo build

# Build in release mode
cargo build --release

# Run the project
cargo run

# Check code without building
cargo check

# Clean build artifacts
cargo clean
```

### Testing
```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests in a specific module
cargo test module_name::
```

### Code Quality
```bash
# Run the Rust linter
cargo clippy

# Apply clippy suggestions
cargo clippy --fix

# Format code
cargo fmt

# Check formatting without applying
cargo fmt -- --check
```

## Architecture and Structure

### API Specification
- `spec/swagger.v3.json` - Contains the complete JIRA API v3 OpenAPI specification (66,130 lines)
- Use this as the authoritative reference for API endpoints, request/response structures, and data models

### Required API Implementations
The following endpoints need to be implemented (from `spec/todo.md`):
- **Search**: GET/POST `/rest/api/3/search` - Search for issues using JQL
- **Projects**: GET `/rest/api/3/project` - Retrieve project information
- **Fields**: GET `/rest/api/3/field` - Get field definitions
- **Issue Types**: GET `/rest/api/3/issuetype` - List issue types
- **Priorities**: GET `/rest/api/3/priority` - List priorities
- **Status Categories**: GET `/rest/api/3/statuscategory` - Get status categories
- **User Search**: GET `/rest/api/3/users/search` - Search for users

### Recommended Project Structure
```
src/
├── lib.rs           # Library root, exports public API
├── client.rs        # Main JIRA client implementation
├── models/          # Data models matching JIRA API responses
│   ├── mod.rs
│   ├── issue.rs
│   ├── project.rs
│   ├── user.rs
│   └── ...
├── api/             # API endpoint implementations
│   ├── mod.rs
│   ├── search.rs
│   ├── projects.rs
│   └── ...
└── error.rs         # Error types and handling
```

### Key Dependencies to Add
When implementing, you'll likely need these dependencies in `Cargo.toml`:
```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
```

### Development Patterns
- Use async/await with Tokio for HTTP requests
- Implement proper error handling with `Result<T, Error>` types
- Deserialize API responses into strongly-typed Rust structs using Serde
- Follow Rust naming conventions (snake_case for functions/variables, CamelCase for types)
- Generate data models from the OpenAPI spec where possible to ensure accuracy