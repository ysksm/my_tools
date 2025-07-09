use crate::error::{Error, Result};
use crate::models::issue::IssueBean;
use crate::storage::IssueStore;
use async_trait::async_trait;
use duckdb::{params, Connection};
use std::path::Path;
use std::sync::{Arc, Mutex};
use tokio::task;

pub struct DuckDbIssueStore {
    conn: Arc<Mutex<Connection>>,
}

impl DuckDbIssueStore {
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db_path = db_path.as_ref().to_path_buf();
        
        let conn = task::spawn_blocking(move || -> Result<Connection> {
            let conn = Connection::open(db_path)?;
            Ok(conn)
        })
        .await
        .map_err(|e| Error::DatabaseError(format!("Task join error: {}", e)))??;

        let conn = Arc::new(Mutex::new(conn));
        let store = Self { conn: conn.clone() };
        
        store.initialize_schema().await?;
        
        Ok(store)
    }

    async fn initialize_schema(&self) -> Result<()> {
        let conn = self.conn.clone();
        
        task::spawn_blocking(move || -> Result<()> {
            let conn = conn.lock().unwrap();
            
            conn.execute(
                r#"
                CREATE SEQUENCE IF NOT EXISTS issues_id_seq START 1;
                "#,
                [],
            )?;
            
            conn.execute(
                r#"
                CREATE TABLE IF NOT EXISTS issues (
                    id INTEGER PRIMARY KEY DEFAULT nextval('issues_id_seq'),
                    project_key VARCHAR NOT NULL,
                    issue_key VARCHAR NOT NULL,
                    summary VARCHAR,
                    description VARCHAR,
                    issue_type VARCHAR,
                    status VARCHAR,
                    priority VARCHAR,
                    assignee VARCHAR,
                    reporter VARCHAR,
                    created VARCHAR,
                    updated VARCHAR,
                    data VARCHAR NOT NULL,
                    UNIQUE(project_key, issue_key)
                )
                "#,
                [],
            )?;

            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_project_key ON issues(project_key)",
                [],
            )?;

            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_issue_key ON issues(issue_key)",
                [],
            )?;

            conn.execute(
                "CREATE INDEX IF NOT EXISTS idx_updated ON issues(updated)",
                [],
            )?;

            Ok(())
        })
        .await
        .map_err(|e| Error::DatabaseError(format!("Task join error: {}", e)))??;

        Ok(())
    }
}

#[async_trait]
impl IssueStore for DuckDbIssueStore {
    async fn store_issue(&self, project_key: &str, issue: &IssueBean) -> Result<()> {
        let json_data = serde_json::to_string(issue)?;
        let project_key = project_key.to_string();
        let issue_key = issue.key.clone();
        
        let summary = issue.fields.get("summary")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let description = issue.fields.get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let issue_type = issue.fields.get("issuetype")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let status = issue.fields.get("status")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let priority = issue.fields.get("priority")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let assignee = issue.fields.get("assignee")
            .and_then(|v| v.get("displayName"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let reporter = issue.fields.get("reporter")
            .and_then(|v| v.get("displayName"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let created = issue.fields.get("created")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let updated = issue.fields.get("updated")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        let conn = self.conn.clone();
        
        task::spawn_blocking(move || -> Result<()> {
            let conn = conn.lock().unwrap();
            
            conn.execute(
                r#"
                INSERT INTO issues (
                    project_key, issue_key, summary, description, issue_type,
                    status, priority, assignee, reporter, created, updated, data
                )
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                ON CONFLICT(project_key, issue_key) DO UPDATE SET
                    summary = excluded.summary,
                    description = excluded.description,
                    issue_type = excluded.issue_type,
                    status = excluded.status,
                    priority = excluded.priority,
                    assignee = excluded.assignee,
                    reporter = excluded.reporter,
                    created = excluded.created,
                    updated = excluded.updated,
                    data = excluded.data
                "#,
                params![
                    project_key,
                    issue_key,
                    summary,
                    description,
                    issue_type,
                    status,
                    priority,
                    assignee,
                    reporter,
                    created,
                    updated,
                    json_data
                ],
            )?;

            Ok(())
        })
        .await
        .map_err(|e| Error::DatabaseError(format!("Task join error: {}", e)))??;

        Ok(())
    }

    async fn store_issues(&self, project_key: &str, issues: &[IssueBean]) -> Result<()> {
        for issue in issues {
            self.store_issue(project_key, issue).await?;
        }
        Ok(())
    }

    async fn get_issue(&self, project_key: &str, issue_key: &str) -> Result<Option<IssueBean>> {
        let conn = self.conn.clone();
        let project_key = project_key.to_string();
        let issue_key = issue_key.to_string();
        
        let result = task::spawn_blocking(move || -> Result<Option<IssueBean>> {
            let conn = conn.lock().unwrap();
            
            let mut stmt = conn.prepare(
                "SELECT data FROM issues WHERE project_key = ? AND issue_key = ?"
            )?;
            
            let mut rows = stmt.query(params![project_key, issue_key])?;
            
            if let Some(row) = rows.next()? {
                let json_data: String = row.get(0)?;
                let issue: IssueBean = serde_json::from_str(&json_data)?;
                Ok(Some(issue))
            } else {
                Ok(None)
            }
        })
        .await
        .map_err(|e| Error::DatabaseError(format!("Task join error: {}", e)))??;

        Ok(result)
    }

    async fn get_all_issues(&self, project_key: &str) -> Result<Vec<IssueBean>> {
        let conn = self.conn.clone();
        let project_key = project_key.to_string();
        
        let issues = task::spawn_blocking(move || -> Result<Vec<IssueBean>> {
            let conn = conn.lock().unwrap();
            
            let mut stmt = conn.prepare(
                "SELECT data FROM issues WHERE project_key = ? ORDER BY updated DESC"
            )?;
            
            let rows = stmt.query_map(params![project_key], |row| {
                let json_data: String = row.get(0)?;
                Ok(json_data)
            })?;
            
            let mut issues = Vec::new();
            for row in rows {
                let json_data = row?;
                let issue: IssueBean = serde_json::from_str(&json_data)?;
                issues.push(issue);
            }
            
            Ok(issues)
        })
        .await
        .map_err(|e| Error::DatabaseError(format!("Task join error: {}", e)))??;

        Ok(issues)
    }

    async fn delete_issue(&self, project_key: &str, issue_key: &str) -> Result<()> {
        let conn = self.conn.clone();
        let project_key = project_key.to_string();
        let issue_key = issue_key.to_string();
        
        task::spawn_blocking(move || -> Result<()> {
            let conn = conn.lock().unwrap();
            
            conn.execute(
                "DELETE FROM issues WHERE project_key = ? AND issue_key = ?",
                params![project_key, issue_key],
            )?;

            Ok(())
        })
        .await
        .map_err(|e| Error::DatabaseError(format!("Task join error: {}", e)))??;

        Ok(())
    }

    async fn clear_project(&self, project_key: &str) -> Result<()> {
        let conn = self.conn.clone();
        let project_key = project_key.to_string();
        
        task::spawn_blocking(move || -> Result<()> {
            let conn = conn.lock().unwrap();
            
            conn.execute(
                "DELETE FROM issues WHERE project_key = ?",
                params![project_key],
            )?;

            Ok(())
        })
        .await
        .map_err(|e| Error::DatabaseError(format!("Task join error: {}", e)))??;

        Ok(())
    }
}