use crate::{
    client::JiraClient,
    error::Result,
    models::{SearchRequest, IssueBean},
    sync::{SyncState, SyncResult, ProjectMetadata},
};
use chrono::{DateTime, Utc};
use std::collections::HashSet;

impl JiraClient {
    /// プロジェクトのメタデータを取得
    pub async fn get_project_metadata(&self, project_key: &str) -> Result<ProjectMetadata> {
        let (project, fields, issue_types, priorities, status_categories) = tokio::try_join!(
            self.get_project(project_key),
            self.get_fields(),
            self.get_issue_types(),
            self.get_priorities(),
            self.get_status_categories()
        )?;
        
        Ok(ProjectMetadata {
            project,
            fields,
            issue_types,
            priorities,
            status_categories,
        })
    }
    
    /// 差分同期を実行
    pub async fn sync_issues(
        &self,
        project_key: &str,
        sync_state: Option<SyncState>,
        max_results_per_page: i32,
    ) -> Result<SyncResult> {
        let current_state = sync_state.unwrap_or_else(|| SyncState::new(project_key.to_string()));
        let mut all_issues = Vec::new();
        let mut start_at = 0;
        let is_complete;
        let mut new_excluded_keys = HashSet::new();
        let mut latest_update_time: Option<DateTime<Utc>> = current_state.last_sync_datetime;
        
        // JQL構築
        let jql = self.build_sync_jql(project_key, &current_state)?;
        
        loop {
            let search_request = SearchRequest {
                jql: jql.clone(),
                start_at: Some(start_at),
                max_results: Some(max_results_per_page),
                fields: None,
                expand: Some(vec!["changelog".to_string()]),
                validate_query: None,
            };
            
            let results = self.search_issues_post(&search_request).await?;
            
            // 同一日時内のキー除外処理
            let filtered_issues: Vec<IssueBean> = results.issues
                .into_iter()
                .filter(|issue| {
                    // 更新日時を取得
                    if let Some(updated) = self.get_issue_updated_time(&issue) {
                        let truncated_time = SyncState::truncate_to_hour(updated);
                        
                        // 最後の同期日時と同じ時刻の場合
                        if let Some(last_sync) = current_state.last_sync_datetime {
                            let last_sync_truncated = SyncState::truncate_to_hour(last_sync);
                            if truncated_time == last_sync_truncated {
                                // 既に取得済みのキーは除外
                                if current_state.excluded_keys.contains(&issue.key) {
                                    return false;
                                }
                                // 新しいキーは記録
                                new_excluded_keys.insert(issue.key.clone());
                            }
                        }
                        
                        // 最新の更新時刻を記録
                        if latest_update_time.is_none() || updated > latest_update_time.unwrap() {
                            latest_update_time = Some(updated);
                        }
                    }
                    true
                })
                .collect();
            
            all_issues.extend(filtered_issues);
            
            // ページネーション処理
            if start_at + max_results_per_page >= results.total {
                is_complete = true;
                break;
            }
            
            start_at += max_results_per_page;
        }
        
        // 新しい同期状態を作成
        let new_sync_state = SyncState {
            last_sync_datetime: latest_update_time,
            excluded_keys: if let Some(last_time) = latest_update_time {
                if let Some(old_time) = current_state.last_sync_datetime {
                    if SyncState::truncate_to_hour(last_time) == SyncState::truncate_to_hour(old_time) {
                        // 同じ時刻の場合は既存のキーと新しいキーをマージ
                        current_state.excluded_keys.union(&new_excluded_keys).cloned().collect()
                    } else {
                        // 新しい時刻の場合は新しいキーのみ
                        new_excluded_keys
                    }
                } else {
                    new_excluded_keys
                }
            } else {
                HashSet::new()
            },
            project_key: project_key.to_string(),
        };
        
        Ok(SyncResult {
            total_fetched: all_issues.len(),
            issues: all_issues,
            is_complete,
            new_sync_state,
        })
    }
    
    /// 初回の全データ取得
    pub async fn sync_all_issues(
        &self,
        project_key: &str,
        max_results_per_page: i32,
    ) -> Result<SyncResult> {
        self.sync_issues(project_key, None, max_results_per_page).await
    }
    
    /// 同期用のJQLを構築
    fn build_sync_jql(&self, project_key: &str, sync_state: &SyncState) -> Result<String> {
        let mut jql = format!("project = {}", project_key);
        
        if let Some(last_sync) = sync_state.last_sync_datetime {
            // JIRAは時単位までしか対応しないため、時単位に丸める
            let truncated_time = SyncState::truncate_to_hour(last_sync);
            let time_str = truncated_time.format("%Y-%m-%d %H:%M").to_string();
            jql.push_str(&format!(" AND updated >= \"{}\"", time_str));
        }
        
        // 更新日時の古い順にソート
        jql.push_str(" ORDER BY updated ASC");
        
        Ok(jql)
    }
    
    /// 課題の更新日時を取得
    fn get_issue_updated_time(&self, issue: &IssueBean) -> Option<DateTime<Utc>> {
        issue.fields.get("updated")
            .and_then(|v| v.as_str())
            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
            .map(|dt| dt.with_timezone(&Utc))
    }
}