use crate::error::{Error, Result};
use crate::models::issue::IssueBean;
use crate::storage::IssueStore;
use async_trait::async_trait;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use tokio::task;

pub struct JsonIssueStore {
    base_path: PathBuf,
}

impl JsonIssueStore {
    pub fn new<P: AsRef<Path>>(base_path: P) -> Result<Self> {
        let base_path = base_path.as_ref().to_path_buf();
        fs::create_dir_all(&base_path)?;
        Ok(Self { base_path })
    }

    fn project_dir(&self, project_key: &str) -> PathBuf {
        self.base_path.join(project_key)
    }

    fn issue_path(&self, project_key: &str, issue_key: &str) -> PathBuf {
        self.project_dir(project_key).join(format!("{}.json.gz", issue_key))
    }

    fn write_compressed_json(&self, path: &Path, issue: &IssueBean) -> Result<()> {
        let json_data = serde_json::to_string_pretty(issue)?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(json_data.as_bytes())?;
        let compressed_data = encoder.finish()?;
        fs::write(path, compressed_data)?;
        Ok(())
    }

    fn read_compressed_json(&self, path: &Path) -> Result<IssueBean> {
        let compressed_data = fs::read(path)?;
        let mut decoder = GzDecoder::new(&compressed_data[..]);
        let mut json_data = String::new();
        decoder.read_to_string(&mut json_data)?;
        let issue: IssueBean = serde_json::from_str(&json_data)?;
        Ok(issue)
    }
}

#[async_trait]
impl IssueStore for JsonIssueStore {
    async fn store_issue(&self, project_key: &str, issue: &IssueBean) -> Result<()> {
        let project_dir = self.project_dir(project_key);
        fs::create_dir_all(&project_dir)?;
        
        let issue_path = self.issue_path(project_key, &issue.key);
        let issue_clone = issue.clone();
        let path_clone = issue_path.clone();
        
        task::spawn_blocking(move || {
            let store = JsonIssueStore { base_path: PathBuf::new() };
            store.write_compressed_json(&path_clone, &issue_clone)
        })
        .await
        .map_err(|e| Error::SerializationError(format!("Task join error: {}", e)))??;
        
        Ok(())
    }

    async fn store_issues(&self, project_key: &str, issues: &[IssueBean]) -> Result<()> {
        for issue in issues {
            self.store_issue(project_key, issue).await?;
        }
        Ok(())
    }

    async fn get_issue(&self, project_key: &str, issue_key: &str) -> Result<Option<IssueBean>> {
        let issue_path = self.issue_path(project_key, issue_key);
        
        if !issue_path.exists() {
            return Ok(None);
        }
        
        let path_clone = issue_path.clone();
        let issue = task::spawn_blocking(move || {
            let store = JsonIssueStore { base_path: PathBuf::new() };
            store.read_compressed_json(&path_clone)
        })
        .await
        .map_err(|e| Error::SerializationError(format!("Task join error: {}", e)))??;
        
        Ok(Some(issue))
    }

    async fn get_all_issues(&self, project_key: &str) -> Result<Vec<IssueBean>> {
        let project_dir = self.project_dir(project_key);
        
        if !project_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut issues = Vec::new();
        let entries = fs::read_dir(&project_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("gz") {
                let path_clone = path.clone();
                let issue = task::spawn_blocking(move || {
                    let store = JsonIssueStore { base_path: PathBuf::new() };
                    store.read_compressed_json(&path_clone)
                })
                .await
                .map_err(|e| Error::SerializationError(format!("Task join error: {}", e)))??;
                
                issues.push(issue);
            }
        }
        
        Ok(issues)
    }

    async fn delete_issue(&self, project_key: &str, issue_key: &str) -> Result<()> {
        let issue_path = self.issue_path(project_key, issue_key);
        
        if issue_path.exists() {
            fs::remove_file(issue_path)?;
        }
        
        Ok(())
    }

    async fn clear_project(&self, project_key: &str) -> Result<()> {
        let project_dir = self.project_dir(project_key);
        
        if project_dir.exists() {
            fs::remove_dir_all(project_dir)?;
        }
        
        Ok(())
    }
}