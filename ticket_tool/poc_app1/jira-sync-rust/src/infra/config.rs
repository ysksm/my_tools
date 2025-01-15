use std::fs;

use crate::constants::{CONFIG_FILE_PATH, OUTPUT_DIR, SYNC_FILE_PATH};
use crate::error::{JiraSyncError, Result};
use crate::models::config::{ConnectSetting, ProjectInfo, ProjectSyncData};

pub struct ConfigManager;

impl ConfigManager {
    pub fn exists() -> bool {
        CONFIG_FILE_PATH.exists()
    }

    pub fn load() -> Result<ConnectSetting> {
        let content = fs::read_to_string(&*CONFIG_FILE_PATH)
            .map_err(|e| JiraSyncError::Config(format!("Failed to read config file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| JiraSyncError::Config(format!("Failed to parse config file: {}", e)))
    }

    pub fn save(setting: &ConnectSetting) -> Result<()> {
        if !OUTPUT_DIR.exists() {
            fs::create_dir_all(&*OUTPUT_DIR)
                .map_err(|e| JiraSyncError::Config(format!("Failed to create output directory: {}", e)))?;
        }

        let content = serde_json::to_string_pretty(setting)
            .map_err(|e| JiraSyncError::Config(format!("Failed to serialize config: {}", e)))?;

        fs::write(&*CONFIG_FILE_PATH, content)
            .map_err(|e| JiraSyncError::Config(format!("Failed to write config file: {}", e)))
    }
}

pub struct SyncDataManager;

impl SyncDataManager {
    pub fn exists() -> bool {
        SYNC_FILE_PATH.exists()
    }

    pub fn exists_project(project_key: &str) -> Result<bool> {
        if !Self::exists() {
            return Ok(false);
        }

        let sync_data = Self::load()?;
        Ok(sync_data.iter().any(|p| p.project_key == project_key))
    }

    pub fn load_project(project: &ProjectInfo) -> Result<ProjectSyncData> {
        if !Self::exists() {
            return Ok(ProjectSyncData::create_default(project));
        }

        let sync_data = Self::load()?;
        Ok(sync_data
            .into_iter()
            .find(|p| p.project_key == project.project_key)
            .unwrap_or_else(|| ProjectSyncData::create_default(project)))
    }

    pub fn load() -> Result<Vec<ProjectSyncData>> {
        let content = fs::read_to_string(&*SYNC_FILE_PATH)
            .map_err(|e| JiraSyncError::Config(format!("Failed to read sync file: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| JiraSyncError::Config(format!("Failed to parse sync file: {}", e)))
    }

    pub fn save_project_sync_data(sync_data: &ProjectSyncData) -> Result<()> {
        if !OUTPUT_DIR.exists() {
            fs::create_dir_all(&*OUTPUT_DIR)
                .map_err(|e| JiraSyncError::Config(format!("Failed to create output directory: {}", e)))?;
        }

        let mut project_sync_data_list = if Self::exists() {
            Self::load()?
        } else {
            Vec::new()
        };

        if let Some(index) = project_sync_data_list
            .iter()
            .position(|p| p.project_key == sync_data.project_key)
        {
            project_sync_data_list[index] = sync_data.clone();
        } else {
            project_sync_data_list.push(sync_data.clone());
        }

        let content = serde_json::to_string_pretty(&project_sync_data_list)
            .map_err(|e| JiraSyncError::Config(format!("Failed to serialize sync data: {}", e)))?;

        fs::write(&*SYNC_FILE_PATH, content)
            .map_err(|e| JiraSyncError::Config(format!("Failed to write sync file: {}", e)))
    }
}
