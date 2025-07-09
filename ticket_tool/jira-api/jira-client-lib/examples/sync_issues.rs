use jira_client_lib::{Auth, JiraClient, JiraConfig, FileSyncStateStore, SyncStateStore};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 環境変数から設定を取得
    let base_url = env::var("JIRA_BASE_URL")?;
    let username = env::var("JIRA_USERNAME")?;
    let api_token = env::var("JIRA_API_TOKEN")?;
    let project_key = env::var("JIRA_PROJECT_KEY").unwrap_or_else(|_| "TEST".to_string());
    
    // JIRAクライアントの作成
    let config = JiraConfig::new(
        base_url,
        Auth::Basic { username, api_token },
    )?;
    let client = JiraClient::new(config)?;
    
    // 同期状態ストアの作成（カレントディレクトリに保存）
    let sync_store = FileSyncStateStore::new("./sync_states");
    
    // 1. プロジェクトのメタデータを取得
    println!("Fetching project metadata for {}...", project_key);
    let metadata = client.get_project_metadata(&project_key).await?;
    println!("Project: {} ({})", metadata.project.name, metadata.project.key);
    println!("Fields: {} found", metadata.fields.len());
    println!("Issue Types: {} found", metadata.issue_types.len());
    println!("Priorities: {} found", metadata.priorities.len());
    println!("Status Categories: {} found\n", metadata.status_categories.len());
    
    // 2. 前回の同期状態を読み込み
    let previous_state = sync_store.load(&project_key).await?;
    
    if let Some(ref state) = previous_state {
        println!("Previous sync state found:");
        println!("  Last sync: {:?}", state.last_sync_datetime);
        println!("  Excluded keys: {} keys\n", state.excluded_keys.len());
    } else {
        println!("No previous sync state found. This will be the initial sync.\n");
    }
    
    // 3. 課題の同期を実行
    println!("Starting issue sync...");
    let sync_result = client.sync_issues(
        &project_key,
        previous_state,
        50, // ページあたり50件
    ).await?;
    
    println!("\nSync completed!");
    println!("  Issues fetched: {}", sync_result.total_fetched);
    println!("  Is complete: {}", sync_result.is_complete);
    
    if let Some(last_update) = sync_result.new_sync_state.last_sync_datetime {
        println!("  Latest update time: {}", last_update);
    }
    
    // 4. 取得した課題の概要を表示
    if !sync_result.issues.is_empty() {
        println!("\nFetched issues:");
        for (i, issue) in sync_result.issues.iter().take(10).enumerate() {
            let summary = issue.fields.get("summary")
                .and_then(|v| v.as_str())
                .unwrap_or("No summary");
            let updated = issue.fields.get("updated")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown");
            
            println!("  {}. {} - {} (Updated: {})", i + 1, issue.key, summary, updated);
        }
        
        if sync_result.issues.len() > 10 {
            println!("  ... and {} more issues", sync_result.issues.len() - 10);
        }
    }
    
    // 5. 新しい同期状態を保存
    sync_store.save(&sync_result.new_sync_state).await?;
    println!("\nSync state saved for next run.");
    
    // 6. 差分同期のデモ（2回目の実行をシミュレート）
    println!("\n--- Simulating incremental sync ---");
    let second_sync = client.sync_issues(
        &project_key,
        Some(sync_result.new_sync_state.clone()),
        50,
    ).await?;
    
    println!("Incremental sync completed!");
    println!("  New issues fetched: {}", second_sync.total_fetched);
    
    Ok(())
}