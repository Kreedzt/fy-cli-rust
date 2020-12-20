use serde::{Deserialize, Serialize};
use std::default::Default;

// 交互的响应
pub enum UserRes {
    QUERY,
    SETKEY
}

#[derive(Serialize, Debug)]
pub struct Params {
    pub q: String,
    pub salt: String,
    pub from: String,
    pub to: String,
    #[serde(rename = "appKey")]
    pub app_key: String,
    pub sign: String,
    #[serde(rename = "signType")]
    pub sign_type: String,
    pub curtime: String,
    pub ext: Option<String>,
    pub voice: Option<String>,
    pub strict: Option<bool>,
    pub vocabld: Option<String>,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            q: String::new(),
            salt: String::new(),
            from: "auto".to_string(),
            to: "auto".to_string(),
            app_key: String::new(),
            sign: String::new(),
            sign_type: "v3".to_string(),
            curtime: String::new(),
            ext: None,
            voice: None,
            strict: None,
            vocabld: None,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct BasicObj {
    pub explains: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct WebItem {
    pub key: String,
    pub value: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct DictObj {
    url: String,
}

#[derive(Deserialize, Debug)]
struct WebDictObj {
    url: String,
}

#[derive(Deserialize, Debug)]
pub struct TransformRes {
    #[serde(rename = "errorCode")]
    pub error_code: String,
    pub query: Option<String>,
    translation: Option<Vec<String>>,
    pub basic: Option<BasicObj>,
    pub web: Option<Vec<WebItem>>,
    l: String,
    dict: Option<DictObj>,
    webdict: Option<WebDictObj>,
    #[serde(rename = "tSpeakUrl")]
    t_speak_url: Option<String>,
    #[serde(rename = "speakUrl")]
    speak_url: Option<String>,
    #[serde(rename = "returnPhrase")]
    return_phrase: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserKey {
    pub app_key: String,
    pub app_secure: String,
}

impl Default for UserKey {
    fn default() -> Self {
        UserKey {
            app_key: String::new(),
            app_secure: String::new(),
        }
    }
}
