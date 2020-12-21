use fy_cli_rust::key::{get_user_key, set_user_key};
use fy_cli_rust::model::{UserKey};
    
#[test]
fn it_key_file() {
    let v = vec![String::new(), String::new()];
    set_user_key(v);
    let user_key: UserKey = get_user_key().unwrap();
    assert_eq!(user_key.app_key, String::new());
    assert_eq!(user_key.app_secure, String::new());
}
