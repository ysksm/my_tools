use env_logger::Env;
use jira_sync_rust::{
    infra::config::ConfigManager,
    models::config::{ConnectSetting, ProjectInfo},
    JiraSync,
};
use log::{error, info};

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Check if config exists
    if !ConfigManager::exists() {
        let default_config = ConnectSetting::create_default();
        if let Err(e) = ConfigManager::save(&default_config) {
            error!("Failed to create default config: {}", e);
            return;
        }
        info!("Please set up your configuration file.");
        return;
    }

    // Load config
    let config = match ConfigManager::load() {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to load config: {}", e);
            return;
        }
    };

    // Initialize JiraSync
    let jira_sync = match JiraSync::new(config.clone()) {
        Ok(sync) => sync,
        Err(e) => {
            error!("Failed to initialize JiraSync: {}", e);
            return;
        }
    };

    // Get projects and update config if project_infos is empty
    if config.project_infos.is_empty() {
        info!("Fetching project list from Jira...");
        match jira_sync.get_projects().await {
            Ok(projects) => {
                let mut updated_config = config.clone();
                updated_config.project_infos = projects
                    .into_iter()
                    .map(|p| ProjectInfo {
                        project_key: p.key.clone(),
                        project_name: p.name.clone(),
                        is_sync: false,
                        where_condition: format!("project = '{}'", p.key),
                        order_by: "updated DESC".to_string(),
                    })
                    .collect();

                if let Err(e) = ConfigManager::save(&updated_config) {
                    error!("Failed to save updated config with projects: {}", e);
                    return;
                }
                info!("Project list has been saved to config file.");
                return;
            }
            Err(e) => {
                error!("Failed to fetch projects: {}", e);
                return;
            }
        }
    }

    // Run sync
    if let Err(e) = jira_sync.sync().await {
        error!("Failed to sync: {}", e);
        return;
    }

    info!("Sync completed successfully.");
}
