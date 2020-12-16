use crate::model::UserKey;

pub fn get_user_key() -> UserKey {
    UserKey {
        appKey: "KEY".to_string(),
        appSecure: "SECURE".to_string(),
    }
}

pub fn set_user_key(app_key: String, app_secure: String) {
    println!("app_key: {}, app_secure: {}", app_key, app_secure);
}
