use std::path::Path;
use duckdb::Connection;
use crate::error::{JiraSyncError, Result};
use crate::models::jira::{Field, FieldSchema};

pub struct DatabaseManager {
    connection: Connection,
}

impl DatabaseManager {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let connection = Connection::open(path)
            .map_err(|e| JiraSyncError::Database(e))?;
        Ok(Self { connection })
    }

    pub async fn initialize_tables(&self, fields: &[Field]) -> Result<()> {
        self.create_status_category_table()?;
        self.create_issue_types_table()?;
        self.create_projects_table()?;
        self.create_priorities_table()?;
        self.create_users_table()?;
        self.create_issues_table(fields)?;

        Ok(())
    }

    fn create_status_category_table(&self) -> Result<()> {
        self.connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS statuscategory (
                    self VARCHAR,
                    id LONG PRIMARY KEY,
                    key VARCHAR,
                    colorName VARCHAR,
                    name VARCHAR
                );"
            )
            .map_err(|e| JiraSyncError::Database(e))?;
        Ok(())
    }

    fn create_issue_types_table(&self) -> Result<()> {
        self.connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS issuetypes (
                    self VARCHAR,
                    id LONG PRIMARY KEY,
                    description VARCHAR,
                    iconUrl VARCHAR,
                    name VARCHAR,
                    untranslatedName VARCHAR,
                    subtask BOOLEAN,
                    avatarId LONG,
                    hierarchyLevel LONG
                );"
            )
            .map_err(|e| JiraSyncError::Database(e))?;
        Ok(())
    }

    fn create_projects_table(&self) -> Result<()> {
        self.connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS projects (
                    expand VARCHAR,
                    self VARCHAR,
                    id LONG PRIMARY KEY,
                    key VARCHAR,
                    name VARCHAR,
                    avatarUrls JSON,
                    projectTypeKey VARCHAR,
                    simplified BOOLEAN,
                    style VARCHAR,
                    isPrivate BOOLEAN
                );"
            )
            .map_err(|e| JiraSyncError::Database(e))?;
        Ok(())
    }

    fn create_priorities_table(&self) -> Result<()> {
        self.connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS priorities (
                    self VARCHAR,
                    statusColor VARCHAR,
                    description VARCHAR,
                    iconUrl VARCHAR,
                    name VARCHAR,
                    id LONG PRIMARY KEY
                );"
            )
            .map_err(|e| JiraSyncError::Database(e))?;
        Ok(())
    }

    fn create_users_table(&self) -> Result<()> {
        self.connection
            .execute_batch(
                "CREATE TABLE IF NOT EXISTS users (
                    self VARCHAR,
                    accountId VARCHAR PRIMARY KEY,
                    accountType VARCHAR,
                    emailAddress VARCHAR,
                    avatarUrls VARCHAR,
                    displayName VARCHAR,
                    active BOOLEAN,
                    locale VARCHAR
                );"
            )
            .map_err(|e| JiraSyncError::Database(e))?;
        Ok(())
    }

    fn create_issues_table(&self, fields: &[Field]) -> Result<()> {
        let mut field_definitions = vec![
            "id INTEGER PRIMARY KEY".to_string(),
            "key VARCHAR".to_string(),
            "expand VARCHAR".to_string(),
            "self VARCHAR".to_string(),
            "fields JSON".to_string(),
        ];

        for field in fields {
            if let Some(schema) = &field.schema {
                let field_def = self.get_field_definition(&field.id, schema);
                field_definitions.push(field_def);
            }
        }

        let create_table_sql = format!(
            "CREATE TABLE IF NOT EXISTS issues ({});",
            field_definitions.join(", ")
        );

        self.connection
            .execute_batch(&create_table_sql)
            .map_err(|e| JiraSyncError::Database(e))?;

        // Create view for easier access
        let mut view_fields = vec![
            "id".to_string(),
            "key".to_string(),
            "expand".to_string(),
            "self".to_string(),
        ];

        for field in fields {
            if let Some(schema) = &field.schema {
                let view_field = self.get_view_field_definition(&field.id, &field.name, schema);
                view_fields.push(view_field);
            }
        }

        let create_view_sql = format!(
            "CREATE VIEW IF NOT EXISTS issues_view AS SELECT {} FROM issues;",
            view_fields.join(", ")
        );

        self.connection
            .execute_batch(&create_view_sql)
            .map_err(|e| JiraSyncError::Database(e))?;

        Ok(())
    }

    fn get_field_definition(&self, field_id: &str, schema: &FieldSchema) -> String {
        match schema.r#type.as_str() {
            "number" => format!("{} INTEGER NULL", field_id),
            "string" | "date" | "datetime" => format!("{} {} NULL", field_id, schema.r#type),
            "project" | "issuetype" | "priority" | "status" => format!("{}_id INTEGER NULL", field_id),
            "user" => format!("{}_accountId string NULL", field_id),
            "array" | "any" => format!("{} JSON", field_id),
            _ => format!("{} JSON", field_id),
        }
    }

    fn get_view_field_definition(&self, field_id: &str, field_name: &str, schema: &FieldSchema) -> String {
        match schema.r#type.as_str() {
            "project" | "issuetype" | "priority" | "status" =>
                format!("{}_id as '{}_id'", field_id, field_name),
            "user" => format!("{}_accountId as '{}_accountId'", field_id, field_name),
            _ => format!("{} as '{}'", field_id, field_name),
        }
    }

    pub fn execute(&self, sql: &str) -> Result<()> {
        self.connection
            .execute_batch(sql)
            .map_err(|e| JiraSyncError::Database(e))?;
        Ok(())
    }
}
