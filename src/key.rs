use crate::model::UserKey;
use dirs::home_dir;
use std::fs;

const CONFIG_FILE_NAME: &'static str = "rfy_config.json";

pub fn get_user_key() -> Result<UserKey, Box<dyn std::error::Error + 'static>> {
    match home_dir() {
        Some(mut p) => {
            p.push(CONFIG_FILE_NAME);
            let res: String = String::from_utf8_lossy(&fs::read(p)?).parse()?;
            let parsed_struct: UserKey = serde_json::from_str(&res).unwrap_or_default();

            Ok(parsed_struct)
        }
        None => Ok(UserKey::default()),
    }
}

// TODO
pub fn set_user_key(app_key: String, app_secure: String) {
    println!("app_key: {}, app_secure: {}", app_key, app_secure);
}
