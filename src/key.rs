use crate::model::UserKey;
use dirs::home_dir;
use serde_json;
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

pub fn set_user_key(app_kv: Vec<String>) {
    let appKey = app_kv.get(0).unwrap().clone();
    let appSecure = app_kv.get(1).unwrap().clone();
    if let Some(mut p) = home_dir() {
        let s = UserKey {
            appKey,
            appSecure,
        };
        let json = serde_json::to_string(&s).unwrap();
        p.push(CONFIG_FILE_NAME);
        fs::write(p, json).unwrap();
        println!("设置成功, appKey: {}, appSecure: {}", s.appKey, s.appSecure);
    }
}
