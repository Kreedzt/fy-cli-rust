use crate::model::{Params, TransformRes};

fn generate_param_input(q: String) -> String {
    if (q.len() <= 20) {
        return q;
    }
    q
}

pub fn get_user_input() -> String {
    generate_param_input("Apple".to_string())
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
    if let Some(q) = res.query {
        println!("{}", q);
    }

    if let Some(k) = res.basic {
        println!("基础释义:");
    }

    if let Some(w) = res.web {
        println!("网络释义:");
    }
}
