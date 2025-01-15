use std::path::PathBuf;
use std::env;

lazy_static::lazy_static! {
    pub static ref OUTPUT_DIR: PathBuf = {
        let home = env::var("HOME").expect("HOME environment variable not set");
        PathBuf::from(home).join(".jira-sync")
    };

    pub static ref CONFIG_FILE_PATH: PathBuf = OUTPUT_DIR.join("config.json");
    pub static ref SYNC_FILE_PATH: PathBuf = OUTPUT_DIR.join("sync.json");
    pub static ref DATA_DIR: PathBuf = OUTPUT_DIR.join("data");
    pub static ref FIELDS_FILE_PATH: PathBuf = DATA_DIR.join("fields.json");
    pub static ref USER_FILE_PATH: PathBuf = DATA_DIR.join("users.json");
    pub static ref USER_FILE_PATH2: PathBuf = DATA_DIR.join("users2.json");
    pub static ref USER_FILE_PATH3: PathBuf = DATA_DIR.join("users3.json");
    pub static ref USER_FILE_PATH4: PathBuf = DATA_DIR.join("users4.json");
    pub static ref USER_FILE_PATH5: PathBuf = DATA_DIR.join("users5.json");
}
