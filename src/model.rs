use serde::{Serialize, Deserialize};

#[derive(Debug)]
pub struct Params {
    pub q: String,
    pub salt: String,
    pub from: String,
    pub to: String,
    pub appKey: String,
    pub sign: String,
    pub signType: String,
    pub curtime: u64,
    pub ext: Option<String>,
    pub voice: Option<String>,
    pub strict: Option<bool>,
    pub vocabld: Option<String>,
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
    pub errorCode: String,
    pub query: Option<String>,
    translation: Option<Vec<String>>,
    pub basic: Option<BasicObj>,
    pub web: Option<WebItem>,
    l: String,
    dict: Option<DictObj>,
    webdict: Option<WebDictObj>,
    tSpeakUrl: Option<String>,
    speakUrl: Option<String>,
    returnPhrase: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct UserKey {
    pub appKey: String,
    pub appSecure: String,
}
