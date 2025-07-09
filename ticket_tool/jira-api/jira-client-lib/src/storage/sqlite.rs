use crate::error::Result;
use crate::models::issue::IssueBean;
use crate::storage::IssueStore;
use async_trait::async_trait;
use sqlx::{Pool, Sqlite, SqlitePool};
use std::path::Path;

pub struct SqliteIssueStore {
    pub(crate) pool: Pool<Sqlite>,
}

impl SqliteIssueStore {
    pub async fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let db_path = db_path.as_ref();
        
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let db_url = format!("sqlite:{}", db_path.display());
        let pool = SqlitePool::connect(&db_url).await?;
        
        let store = Self { pool };
        store.initialize_schema().await?;
        
        Ok(store)
    }

    async fn initialize_schema(&self) -> Result<()> {
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
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_project_key ON issues(project_key);
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_issue_key ON issues(issue_key);
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE INDEX IF NOT EXISTS idx_updated ON issues(updated);
            "#,
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[async_trait]
impl IssueStore for SqliteIssueStore {
    async fn store_issue(&self, project_key: &str, issue: &IssueBean) -> Result<()> {
        let json_data = serde_json::to_string(issue)?;
        
        let summary = issue.fields.get("summary")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let description = issue.fields.get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let issue_type = issue.fields.get("issuetype")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let status = issue.fields.get("status")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let priority = issue.fields.get("priority")
            .and_then(|v| v.get("name"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let assignee = issue.fields.get("assignee")
            .and_then(|v| v.get("displayName"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let reporter = issue.fields.get("reporter")
            .and_then(|v| v.get("displayName"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let created = issue.fields.get("created")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let updated = issue.fields.get("updated")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        sqlx::query(
            r#"
            INSERT INTO issues (
                project_key, issue_key, summary, description, issue_type,
                status, priority, assignee, reporter, created, updated, data
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
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
        )
        .bind(project_key)
        .bind(&issue.key)
        .bind(summary)
        .bind(description)
        .bind(issue_type)
        .bind(status)
        .bind(priority)
        .bind(assignee)
        .bind(reporter)
        .bind(created)
        .bind(updated)
        .bind(json_data)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn store_issues(&self, project_key: &str, issues: &[IssueBean]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for issue in issues {
            let json_data = serde_json::to_string(issue)?;
            
            let summary = issue.fields.get("summary")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let description = issue.fields.get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let issue_type = issue.fields.get("issuetype")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let status = issue.fields.get("status")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let priority = issue.fields.get("priority")
                .and_then(|v| v.get("name"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let assignee = issue.fields.get("assignee")
                .and_then(|v| v.get("displayName"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let reporter = issue.fields.get("reporter")
                .and_then(|v| v.get("displayName"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let created = issue.fields.get("created")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let updated = issue.fields.get("updated")
                .and_then(|v| v.as_str())
                .unwrap_or("");

            sqlx::query(
                r#"
                INSERT INTO issues (
                    project_key, issue_key, summary, description, issue_type,
                    status, priority, assignee, reporter, created, updated, data
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
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
            )
            .bind(project_key)
            .bind(&issue.key)
            .bind(summary)
            .bind(description)
            .bind(issue_type)
            .bind(status)
            .bind(priority)
            .bind(assignee)
            .bind(reporter)
            .bind(created)
            .bind(updated)
            .bind(json_data)
            .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    async fn get_issue(&self, project_key: &str, issue_key: &str) -> Result<Option<IssueBean>> {
        let row = sqlx::query_as::<_, (String,)>(
            "SELECT data FROM issues WHERE project_key = $1 AND issue_key = $2"
        )
        .bind(project_key)
        .bind(issue_key)
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some((json_data,)) => {
                let issue: IssueBean = serde_json::from_str(&json_data)?;
                Ok(Some(issue))
            }
            None => Ok(None),
        }
    }

    async fn get_all_issues(&self, project_key: &str) -> Result<Vec<IssueBean>> {
        let rows = sqlx::query_as::<_, (String,)>(
            "SELECT data FROM issues WHERE project_key = $1 ORDER BY updated DESC"
        )
        .bind(project_key)
        .fetch_all(&self.pool)
        .await?;

        let mut issues = Vec::with_capacity(rows.len());
        for (json_data,) in rows {
            let issue: IssueBean = serde_json::from_str(&json_data)?;
            issues.push(issue);
        }

        Ok(issues)
    }

    async fn delete_issue(&self, project_key: &str, issue_key: &str) -> Result<()> {
        sqlx::query("DELETE FROM issues WHERE project_key = $1 AND issue_key = $2")
            .bind(project_key)
            .bind(issue_key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn clear_project(&self, project_key: &str) -> Result<()> {
        sqlx::query("DELETE FROM issues WHERE project_key = $1")
            .bind(project_key)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}