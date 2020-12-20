use crate::model::{Params, TransformRes, UserRes};
use clap::{App, Arg};
use sha2::{Digest, Sha256};
use std::time::SystemTime;
use uuid::Uuid;

fn generate_param_input(q: String) -> String {
    if q.len() <= 20 {
        return q;
    }
    format!("{}{}{}", &q[..10], q.len(), &q[q.len() - 10..])
}

// 获取交互方式
pub fn get_user_way() -> Result<(UserRes, Vec<String>), ()> {
    let matches = App::new("fy-cli-rust")
        .version("0.1")
        .author("Kreedzt <zhaozisong1@live.com>")
        .about("使用有道翻译 api 进行翻译")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .max_values(2)
                .min_values(2)
                .about("设置密钥, 格式: app_key app_secure")
                .required(false)
                .takes_value(true)
                .conflicts_with("content"),
        )
        .arg(
            Arg::new("content")
                .about("待翻译的内容")
                .required_unless_present("config"),
        )
        .get_matches();

    // println!("matches: {:?}", matches);

    if let Some(config_kv) = matches.values_of("config") {
        let v: Vec<String> = config_kv.into_iter().map(|s| s.to_string()).collect();
        return Ok((UserRes::SETKEY, v));
    }

    if let Some(input) = matches.value_of("content") {
        let mut v = Vec::with_capacity(1);
        v.push(input.to_string());
        return Ok((UserRes::QUERY, v));
    }

    Err(())
}

pub fn get_user_input(s: String) -> String {
    generate_param_input(s)
}

pub fn generate_param(user_input: String, app_key: String, app_secure: String) -> Params {
    let salt = format!("{}", Uuid::new_v4());
    let curtime = format!(
        "{}",
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    );

    let source_sign = format!(
        "{}{}{}{}{}",
        app_key,
        generate_param_input(user_input.clone()),
        salt,
        curtime,
        app_secure
    );
    let mut hasher = Sha256::new();
    hasher.update(source_sign.as_bytes());
    let sign: String = format!("{:X}", hasher.finalize());

    Params {
        q: user_input,
        salt,
        from: "auto".to_string(),
        to: "auto".to_string(),
        app_key,
        sign,
        sign_type: "v3".to_string(),
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
        let mut s = String::new();
        for x in k.explains {
            s.push_str(&x);
        }
        println!("{}", s);
        println!("========");
    }

    if let Some(w) = res.web {
        println!("网络释义:");
        for item in w {
            println!("{}", item.key);
            let mut s = String::new();
            for wi in item.value {
                s.push_str(&wi);
            }
            println!("{}", s);
            println!("--------");
        }
    }
}
