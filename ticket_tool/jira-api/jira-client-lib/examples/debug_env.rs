use std::env;

fn main() {
    // 現在のディレクトリを表示
    println!("Current directory: {:?}", env::current_dir().unwrap());
    
    // .envファイルを読み込む
    match dotenv::dotenv() {
        Ok(path) => println!("Loaded .env from: {:?}", path),
        Err(e) => println!("Failed to load .env: {:?}", e),
    }
    
    // 環境変数を確認
    println!("\nEnvironment variables:");
    match env::var("JIRA_URL") {
        Ok(val) => println!("JIRA_URL = {}", val),
        Err(e) => println!("JIRA_URL not found: {:?}", e),
    }
    
    match env::var("JIRA_USER") {
        Ok(val) => println!("JIRA_USER = {}", val),
        Err(e) => println!("JIRA_USER not found: {:?}", e),
    }
    
    match env::var("JIRA_API_TOKEN") {
        Ok(val) => println!("JIRA_API_TOKEN = {} (length: {})", "*".repeat(8), val.len()),
        Err(e) => println!("JIRA_API_TOKEN not found: {:?}", e),
    }
}