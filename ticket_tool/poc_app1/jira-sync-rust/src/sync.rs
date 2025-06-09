use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use log::{info, error};

use crate::constants::DATA_DIR;
use crate::error::{JiraSyncError, Result};
use crate::infra::{
    config::SyncDataManager,
    db::DatabaseManager,
    jira_client::JiraClient,
};
use crate::models::{
    config::{ConnectSetting, ProjectInfo, ProjectSyncData},
    jira::{Issue, SearchRequest, Field, FieldSchema, Project},
};

pub struct JiraSync {
    jira_client: JiraClient,
    config: ConnectSetting,
}

impl JiraSync {
    pub fn new(config: ConnectSetting) -> Result<Self> {
        let jira_client = JiraClient::new(config.credential_info.clone())?;
        Ok(Self {
            jira_client,
            config,
        })
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>> {
        self.jira_client.get_projects().await
    }

    pub async fn sync(&self) -> Result<()> {
        // Get sync projects
        let sync_projects: Vec<_> = self.config.project_infos
            .iter()
            .filter(|p| p.is_sync)
            .collect();

        for project in sync_projects {
            info!("Project: {} {} Data Sync Start.", project.project_key, project.project_name);
            
            let project_dir = DATA_DIR.join(&project.project_key);
            if !project_dir.exists() {
                info!("Creating directory for project: {}", project.project_key);
                fs::create_dir_all(&project_dir)?;
            }

            self.sync_project(&project_dir, project).await?;
        }

        Ok(())
    }

    async fn sync_project(&self, project_dir: &PathBuf, project: &ProjectInfo) -> Result<()> {
        // Initialize database
        info!("Initializing database for project: {}", project.project_key);
        let db = DatabaseManager::new(project_dir.join("data.duckdb"))?;
        
        // Get fields for database schema
        info!("Fetching fields for project: {}", project.project_key);
        let fields = self.jira_client.get_fields().await?;
        if db.initialize_tables(&fields).await.is_err() {
            error!("Failed to initialize database tables for project: {}", project.project_key);
        }

        // Get project sync data
        let mut project_sync_data = SyncDataManager::load_project(project)?;
        let mut total = 0;

        loop {
            let jql = self.create_jql(&project_sync_data, project);
            info!("JQL Query: {}", jql);
            let request = self.create_search_request(&jql);

            let mut response = self.jira_client.search_issues(request).await?;
            if response.issues.is_empty() {
                break;
            }

            let issues_len = response.issues.len();
            for issue in response.issues.drain(..) {
                self.process_issue(project_dir, &issue, &fields, &db)?;
                
                // Update sync data
                let issue_updated: DateTime<Utc> = serde_json::from_value(
                    issue.fields.get("updated")
                        .ok_or_else(|| JiraSyncError::JiraApi("No updated field in issue".to_string()))?
                        .clone(),
                )?;

                if project_sync_data.last_updated.to_rfc3339() != issue_updated.to_rfc3339() {
                    project_sync_data.last_updated_issue_keys.clear();
                }
                project_sync_data.last_updated_issue_keys.push(issue.key);
                project_sync_data.last_updated = issue_updated;
            }

            info!(
                "Project: {} {} Processed {} issues.",
                project.project_key, project.project_name, issues_len
            );

            let last_updated_issue_keys = self.get_last_updated_issue_keys(&db, project_sync_data.last_updated)?;
            project_sync_data.last_updated_issue_keys = last_updated_issue_keys;

            info!(
                "Project: {} {} Processed {} issues. Total: {}",
                project.project_key, project.project_name, issues_len, total + issues_len
            );

            SyncDataManager::save_project_sync_data(&project_sync_data)?;
            total += issues_len;

            if response.total == 0 || response.total == response.max_results {
                info!(
                    "Project: {} {} Data Sync Completed. Total: {}",
                    project.project_key, project.project_name, total
                );
                break;
            }
        }

        Ok(())
    }

    fn get_last_updated_issue_keys(&self, db: &DatabaseManager, last_updated: DateTime<Utc>) -> Result<Vec<String>> {
        let last_updated_hour = last_updated.format("%Y-%m-%d %H:00").to_string();
        let last_updated_issue_keys_sql = format!("SELECT Key FROM issues WHERE updated >= '{}';", last_updated_hour);
        info!("Executing SQL to get last updated issue keys: {}", last_updated_issue_keys_sql);
        let results = db.query(&last_updated_issue_keys_sql)?;
        info!("Last updated issue keys: {:?}", results);
        Ok(results.into_iter().map(|(key)| key).collect())
    }

    fn process_issue(
        &self,
        project_dir: &PathBuf,
        issue: &Issue,
        fields: &[Field],
        db: &DatabaseManager,
    ) -> Result<()> {
        // Write issue JSON
        let issue_file_path = project_dir.join(format!("{}.json", issue.key));
        fs::write(
            &issue_file_path,
            serde_json::to_string_pretty(issue)?,
        )?;

        // Insert into database
        let insert_sql = self.create_insert_sql(issue, fields)?;
        // println!("Insert SQL: {}", insert_sql);
        db.execute(&insert_sql)?;

        Ok(())
    }

    fn create_jql(&self, sync_data: &ProjectSyncData, project: &ProjectInfo) -> String {
        let mut conditions = vec![project.where_condition.clone()];
        
        // Add updated condition
        // let last_updated = sync_data.last_updated.format("%Y-%m-%d %H:%M");
        let last_updated = sync_data.last_updated.format("%Y-%m-%d %H:00");
        conditions.push(format!("updated >= '{}'", last_updated));

        // Add exclude keys condition
        if !sync_data.last_updated_issue_keys.is_empty() {
            let keys = sync_data.last_updated_issue_keys
                .iter()
                .map(|k| format!("'{}'", k))
                .collect::<Vec<_>>()
                .join(",");
            conditions.push(format!("key NOT IN ({})", keys));
        }

        format!("{} ORDER BY {}", conditions.join(" AND "), project.order_by)
    }

    fn create_search_request(&self, jql: &str) -> SearchRequest {
        SearchRequest {
            jql: jql.to_string(),
            start_at: 0,
            max_results: 100,
            fields: vec!["*all".to_string()],
            expand: vec!["changelog".to_string()],
        }
    }

    fn create_insert_sql(&self, issue: &Issue, fields: &[Field]) -> Result<String> {
        let mut field_names = Vec::new();
        let mut field_values = Vec::new();

        // Add base fields
        field_names.extend(["id", "key", "expand", "self"].iter().map(|s| s.to_string()));
        field_values.extend_from_slice(&[
            format!("'{}'", issue.id),
            format!("'{}'", issue.key),
            issue.expand.as_ref().map_or("NULL".to_string(), |e| format!("'{}'", e)),
            format!("'{}'", issue.self_link),
        ]);

        // Add custom fields
        for field in fields {
            if let Some(schema) = &field.schema {
                if let Some(value) = issue.fields.get(&field.id) {
                    let (name, value) = self.get_field_value(&field.id, schema, value)?;
                    if !name.is_empty() {
                        field_names.push(name);
                        field_values.push(value);
                    }
                }
            }
        }

        Ok(format!(
            "INSERT OR REPLACE INTO issues ({}) VALUES ({});",
            field_names.join(","),
            field_values.join(",")
        ))
    }

    fn get_field_value(
        &self,
        field_id: &str,
        schema: &FieldSchema,
        value: &serde_json::Value,
    ) -> Result<(String, String)> {

        // println!("field_id: {}, {}", field_id, schema.r#type.as_str());

        let new_field_name = match schema.r#type.as_str() {
            "string" | "date" | "datetime" => field_id.to_string(),
            "number" => field_id.to_string(),
            "project" | "issuetype" | "priority" | "status" => format!("{}_id", field_id),
            "user" => format!("{}_accountId", field_id),
            _ => field_id.to_string(),
        };

        if value.is_null() {
            return Ok((new_field_name, "null".to_string()));
        }

        match schema.r#type.as_str() {
            "string" | "date" | "datetime" => {
                Ok((
                    field_id.to_string(),
                    format!("'{}'", value.as_str().unwrap_or_default().replace('\'', "''")),
                ))
            }
            "number" => Ok((field_id.to_string(), value.to_string())),
            "project" | "issuetype" | "priority" | "status" => {
                let id = value.get("id").and_then(|v| v.as_str()).unwrap_or_default();
                Ok((format!("{}_id", field_id), format!("'{}'", id)))
            }
            "user" => {
                let account_id = value
                    .get("accountId")
                    .and_then(|v| v.as_str())
                    .unwrap_or_default();
                Ok((
                    format!("{}_accountId", field_id),
                    format!("'{}'", account_id),
                ))
            }
            "array" | "any" => Ok((
                field_id.to_string(),
                format!("'{}'", value.to_string().replace('\'', "''")),
            )),
            _ => Ok((
                field_id.to_string(),
                format!("'{}'", value.to_string().replace('\'', "''")),
            )),
        }
    }
}
