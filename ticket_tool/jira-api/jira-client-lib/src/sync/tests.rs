#[cfg(test)]
mod tests {
    use super::super::*;
    use chrono::{TimeZone, Utc};
    use tempfile::TempDir;
    
    #[test]
    fn test_truncate_to_hour() {
        let dt = Utc.with_ymd_and_hms(2024, 1, 15, 14, 35, 22).unwrap();
        let truncated = SyncState::truncate_to_hour(dt);
        
        assert_eq!(truncated.hour(), 14);
        assert_eq!(truncated.minute(), 0);
        assert_eq!(truncated.second(), 0);
    }
    
    #[test]
    fn test_sync_state_new() {
        let state = SyncState::new("TEST".to_string());
        
        assert_eq!(state.project_key, "TEST");
        assert!(state.last_sync_datetime.is_none());
        assert!(state.excluded_keys.is_empty());
    }
    
    #[tokio::test]
    async fn test_file_sync_state_store() {
        let temp_dir = TempDir::new().unwrap();
        let store = FileSyncStateStore::new(temp_dir.path());
        
        // 初期状態では何も保存されていない
        let loaded = store.load("TEST").await.unwrap();
        assert!(loaded.is_none());
        
        // 状態を作成して保存
        let mut state = SyncState::new("TEST".to_string());
        state.last_sync_datetime = Some(Utc::now());
        state.excluded_keys.insert("TEST-1".to_string());
        state.excluded_keys.insert("TEST-2".to_string());
        
        store.save(&state).await.unwrap();
        
        // 保存した状態を読み込み
        let loaded = store.load("TEST").await.unwrap().unwrap();
        assert_eq!(loaded.project_key, "TEST");
        assert!(loaded.last_sync_datetime.is_some());
        assert_eq!(loaded.excluded_keys.len(), 2);
        assert!(loaded.excluded_keys.contains("TEST-1"));
        assert!(loaded.excluded_keys.contains("TEST-2"));
    }
    
    #[tokio::test]
    async fn test_sync_state_serialization() {
        let mut state = SyncState::new("PROJECT".to_string());
        state.last_sync_datetime = Some(Utc.with_ymd_and_hms(2024, 1, 15, 10, 0, 0).unwrap());
        state.excluded_keys.insert("KEY-1".to_string());
        
        // シリアライズ
        let json = serde_json::to_string(&state).unwrap();
        
        // デシリアライズ
        let deserialized: SyncState = serde_json::from_str(&json).unwrap();
        
        assert_eq!(state.project_key, deserialized.project_key);
        assert_eq!(state.last_sync_datetime, deserialized.last_sync_datetime);
        assert_eq!(state.excluded_keys, deserialized.excluded_keys);
    }
}