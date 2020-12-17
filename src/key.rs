use crate::model::UserKey;
use dirs::home_dir;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

const config_file_name: &'static str = "rfy_config.json";

pub fn get_user_key() -> Result<UserKey, Box<dyn std::error::Error + 'static>> {
    if let Some(mut p) = home_dir() {
        p.push(config_file_name);
        let res: String = String::from_utf8_lossy(&fs::read(p)?).parse()?;
        let parsedStruct: UserKey = serde_json::from_str(&res).unwrap_or(UserKey::new());

        Ok(parsedStruct)
    } else {
        Ok(UserKey::new())
    }
}

pub fn set_user_key(app_key: String, app_secure: String) {
    println!("app_key: {}, app_secure: {}", app_key, app_secure);
}
