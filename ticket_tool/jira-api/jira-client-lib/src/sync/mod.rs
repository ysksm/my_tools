use crate::{error::Result, models::{IssueBean, FieldDetails, IssueTypeDetails, Priority, StatusCategory, Project}};
use chrono::{DateTime, Utc, Timelike};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;
use async_trait::async_trait;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    /// 最後に同期した日時（分単位まで）
    pub last_sync_datetime: Option<DateTime<Utc>>,
    /// 最後の同期日時と同じ時刻（時まで）の課題キー
    pub excluded_keys: HashSet<String>,
    /// 同期したプロジェクトキー
    pub project_key: String,
}

impl SyncState {
    pub fn new(project_key: String) -> Self {
        Self {
            last_sync_datetime: None,
            excluded_keys: HashSet::new(),
            project_key,
        }
    }
    
    /// 日時を時単位に丸める（JIRAのAPIは分秒を考慮しないため）
    pub fn truncate_to_hour(dt: DateTime<Utc>) -> DateTime<Utc> {
        dt.date_naive()
            .and_hms_opt(dt.hour(), 0, 0)
            .unwrap()
            .and_utc()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub fields: Vec<FieldDetails>,
    pub issue_types: Vec<IssueTypeDetails>,
    pub priorities: Vec<Priority>,
    pub status_categories: Vec<StatusCategory>,
    pub project: Project,
}

#[async_trait]
pub trait SyncStateStore: Send + Sync {
    /// 同期状態を保存
    async fn save(&self, state: &SyncState) -> Result<()>;
    
    /// 同期状態を読み込み
    async fn load(&self, project_key: &str) -> Result<Option<SyncState>>;
}

/// ファイルベースの同期状態ストア
pub struct FileSyncStateStore {
    base_path: std::path::PathBuf,
}

impl FileSyncStateStore {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }
    
    fn state_file_path(&self, project_key: &str) -> std::path::PathBuf {
        self.base_path.join(format!("sync_state_{}.json", project_key))
    }
}

#[async_trait]
impl SyncStateStore for FileSyncStateStore {
    async fn save(&self, state: &SyncState) -> Result<()> {
        let path = self.state_file_path(&state.project_key);
        let content = serde_json::to_string_pretty(state)?;
        tokio::fs::create_dir_all(&self.base_path).await?;
        tokio::fs::write(path, content).await?;
        Ok(())
    }
    
    async fn load(&self, project_key: &str) -> Result<Option<SyncState>> {
        let path = self.state_file_path(project_key);
        if !path.exists() {
            return Ok(None);
        }
        
        let content = tokio::fs::read_to_string(path).await?;
        let state = serde_json::from_str(&content)?;
        Ok(Some(state))
    }
}

#[derive(Debug)]
pub struct SyncResult {
    pub issues: Vec<IssueBean>,
    pub total_fetched: usize,
    pub is_complete: bool,
    pub new_sync_state: SyncState,
}