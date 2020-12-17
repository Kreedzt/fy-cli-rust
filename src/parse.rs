use crate::model::{Params, TransformRes};
use sha2::Sha256;

fn generate_param_input(q: String) -> String {
    if (q.len() <= 20) {
        return q;
    }
    format!("{}{}{}", &q[..10], q.len(), &q[q.len() - 10..])
}

pub fn get_user_input() -> String {
    generate_param_input("Apple".to_string())
}

pub fn generate_param(user_input: String, app_key: String, app_secure: String) -> Params {
    let salt = "zz".to_string();
    let curtime = 1;

    let source_sign = format!(
        "{}{}{}{}{}",
        app_key,
        generate_param_input(user_input),
        salt,
        curtime,
        app_secure
    );
    let mut hasher = Sha256::new();
    hasher.update(source_sign.as_bytes());
    let sign = hasher.finalize();

    Params {
        q: user_input,
        salt,
        from: "auto".to_string(),
        to: "auto".to_string(),
        appKey: app_key,
        sign,
        signType: "v3".to_string(),
        curtime,
        ..Params::default()
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
