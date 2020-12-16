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

pub struct BasicObj {
    pub explains: Vec<String>,
}

pub struct WebItem {
    pub key: String,
    pub value: Vec<String>,
}

struct DictObj {
    url: String,
}

struct WebDictObj {
    url: String,
}

pub struct TransformRes {
    pub errorCode: String,
    pub query: String,
    translation: Vec<String>,
    pub basic: Option<BasicObj>,
    pub web: Option<WebItem>,
    l: String,
    dict: DictObj,
    webdict: WebDictObj,
    tSpeakUrl: String,
    speakUrl: String,
    returnPhrase: Vec<String>,
}

#[derive(Debug)]
pub struct UserKey {
    pub appKey: String,
    pub appSecure: String,
}
