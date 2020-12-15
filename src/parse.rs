pub struct Params {
    q: String,
    salt: String,
    from: String,
    to: String,
    appKey: String,
    sign: String,
    signType: String,
    curtime: u64,
    ext: Option<String>,
    voice: Option<String>,
    strict: Option<bool>,
    vocabld: Option<String>,
}

struct BasicObj {
    explains: Vec<String>,
}

struct WebItem {
    key: String,
    value: Vec<String>,
}

struct DictObj {
    url: String,
}

struct WebDictObj {
    url: String,
}

pub struct TransformRes {
    errorCode: String,
    query: String,
    translation: Vec<String>,
    basic: Option<BasicObj>,
    web: Option<WebItem>,
    l: String,
    dict: DictObj,
    webdict: WebDictObj,
    tSpeakUrl: String,
    speakUrl: String,
    returnPhrase: Vec<String>,
}

fn generate_param_input(q: String) -> String {
    if (q.len() <= 20) {
        return q;
    }
    q
}

pub fn get_user_input() -> String {
    generate_param_input("".to_string())
}

pub fn generate_param(user_input: String, app_key: String, app_secure: String) -> Params {
    let sign = "".to_string();
    let curtime = 1;

    Params {
        q: user_input,
        salt: "".to_string(),
        from: "auto".to_string(),
        to: "auto".to_string(),
        appKey: app_key,
        sign,
        signType: "v3".to_string(),
        curtime,
        ext: None,
        voice: None,
        strict: None,
        vocabld: None,
    }
}

pub fn display_res(res: TransformRes) {
    println!("{}", res.query);

    if let Some(k) = res.basic {
        println!("基础释义:");
    }

    if let Some(w) = res.web {
        println!("网络释义:");
    }
}
