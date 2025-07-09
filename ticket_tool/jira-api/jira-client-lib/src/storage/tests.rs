#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::issue::IssueBean;
    use serde_json::json;
    use std::collections::HashMap;
    use tempfile::TempDir;

    fn create_test_issue(key: &str) -> IssueBean {
        let mut fields = HashMap::new();
        fields.insert("summary".to_string(), json!("Test Issue Summary"));
        fields.insert("description".to_string(), json!("Test Issue Description"));
        fields.insert("issuetype".to_string(), json!({
            "name": "Bug",
            "id": "1"
        }));
        fields.insert("status".to_string(), json!({
            "name": "Open",
            "id": "1"
        }));
        fields.insert("priority".to_string(), json!({
            "name": "High",
            "id": "1"
        }));
        fields.insert("assignee".to_string(), json!({
            "displayName": "John Doe",
            "emailAddress": "john@example.com"
        }));
        fields.insert("reporter".to_string(), json!({
            "displayName": "Jane Smith",
            "emailAddress": "jane@example.com"
        }));
        fields.insert("created".to_string(), json!("2024-01-01T10:00:00.000+0000"));
        fields.insert("updated".to_string(), json!("2024-01-02T15:30:00.000+0000"));

        IssueBean {
            id: "12345".to_string(),
            key: key.to_string(),
            self_url: format!("https://example.atlassian.net/rest/api/2/issue/{}", key),
            expand: None,
            fields,
        }
    }

    #[tokio::test]
    async fn test_json_store_operations() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonIssueStore::new(temp_dir.path()).unwrap();
        
        let project_key = "TEST";
        let issue = create_test_issue("TEST-123");
        
        store.store_issue(project_key, &issue).await.unwrap();
        
        let retrieved = store.get_issue(project_key, "TEST-123").await.unwrap();
        assert!(retrieved.is_some());
        
        let retrieved_issue = retrieved.unwrap();
        assert_eq!(retrieved_issue.key, "TEST-123");
        assert_eq!(retrieved_issue.id, "12345");
        
        let summary = retrieved_issue.fields.get("summary")
            .and_then(|v| v.as_str())
            .unwrap();
        assert_eq!(summary, "Test Issue Summary");
        
        let all_issues = store.get_all_issues(project_key).await.unwrap();
        assert_eq!(all_issues.len(), 1);
        
        store.delete_issue(project_key, "TEST-123").await.unwrap();
        
        let deleted = store.get_issue(project_key, "TEST-123").await.unwrap();
        assert!(deleted.is_none());
    }

    #[tokio::test]
    async fn test_json_store_multiple_issues() {
        let temp_dir = TempDir::new().unwrap();
        let store = JsonIssueStore::new(temp_dir.path()).unwrap();
        
        let project_key = "TEST";
        let issues = vec![
            create_test_issue("TEST-001"),
            create_test_issue("TEST-002"),
            create_test_issue("TEST-003"),
        ];
        
        store.store_issues(project_key, &issues).await.unwrap();
        
        let all_issues = store.get_all_issues(project_key).await.unwrap();
        assert_eq!(all_issues.len(), 3);
        
        let issue_keys: Vec<String> = all_issues.iter().map(|i| i.key.clone()).collect();
        assert!(issue_keys.contains(&"TEST-001".to_string()));
        assert!(issue_keys.contains(&"TEST-002".to_string()));
        assert!(issue_keys.contains(&"TEST-003".to_string()));
        
        store.clear_project(project_key).await.unwrap();
        
        let cleared = store.get_all_issues(project_key).await.unwrap();
        assert_eq!(cleared.len(), 0);
    }

    #[tokio::test]
    async fn test_sqlite_store_operations() {
        use crate::storage::sqlite::SqliteIssueStore;
        use sqlx::sqlite::SqlitePool;
        
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS issues (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                project_key TEXT NOT NULL,
                issue_key TEXT NOT NULL,
                summary TEXT,
                description TEXT,
                issue_type TEXT,
                status TEXT,
                priority TEXT,
                assignee TEXT,
                reporter TEXT,
                created TEXT,
                updated TEXT,
                data TEXT NOT NULL,
                UNIQUE(project_key, issue_key)
            )
            "#,
        )
        .execute(&pool)
        .await
        .unwrap();
        
        let store = SqliteIssueStore { pool };
        
        let project_key = "TEST";
        let issue = create_test_issue("TEST-456");
        
        store.store_issue(project_key, &issue).await.unwrap();
        
        let retrieved = store.get_issue(project_key, "TEST-456").await.unwrap();
        assert!(retrieved.is_some());
        
        let retrieved_issue = retrieved.unwrap();
        assert_eq!(retrieved_issue.key, "TEST-456");
        
        let all_issues = store.get_all_issues(project_key).await.unwrap();
        assert_eq!(all_issues.len(), 1);
        
        let updated_issue = create_test_issue("TEST-456");
        store.store_issue(project_key, &updated_issue).await.unwrap();
        
        let all_issues_after_update = store.get_all_issues(project_key).await.unwrap();
        assert_eq!(all_issues_after_update.len(), 1);
    }

    #[tokio::test]
    async fn test_duckdb_store_operations() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.duckdb");
        let store = DuckDbIssueStore::new(&db_path).await.unwrap();
        
        let project_key = "TEST";
        let issue = create_test_issue("TEST-789");
        
        store.store_issue(project_key, &issue).await.unwrap();
        
        let retrieved = store.get_issue(project_key, "TEST-789").await.unwrap();
        assert!(retrieved.is_some());
        
        let retrieved_issue = retrieved.unwrap();
        assert_eq!(retrieved_issue.key, "TEST-789");
        
        let all_issues = store.get_all_issues(project_key).await.unwrap();
        assert_eq!(all_issues.len(), 1);
        
        store.delete_issue(project_key, "TEST-789").await.unwrap();
        
        let deleted = store.get_issue(project_key, "TEST-789").await.unwrap();
        assert!(deleted.is_none());
    }
}