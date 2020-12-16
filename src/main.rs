use fy_cli_rust::key::get_user_key;
use fy_cli_rust::parse::{generate_param, get_user_input};
// use crate::utils::parse::display_res;
// use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = get_user_input();
    println!("input: {}", input);

    let user_key = get_user_key();
    println!("user_key: {:?}", user_key);

    let params = generate_param(input, user_key.appKey, user_key.appSecure);
    println!("params: {:?}", params);
    
    println!("main fn effected");
    // let resp = reqwest::get("https://httpbin.org/ip")
    //     .await?
    //     .json::<HashMap<String, String>>()
    //     .await?;
    // println!("{:#?}", resp);
    Ok(())
}
