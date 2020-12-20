use fy_cli_rust::key::{get_user_key, set_user_key};
use fy_cli_rust::model::{TransformRes, UserRes};
use fy_cli_rust::parse::{display_res, generate_param, get_user_input, get_user_way};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let way = get_user_way().unwrap();

    match way.0 {
        UserRes::SET_KEY => {
            set_user_key(way.1);
            return Ok(())
        },
        _ => {}
    }
    
    let input = get_user_input(way.1.get(0).unwrap().clone());

    let user_key = get_user_key()?;

    let params = generate_param(input, user_key.appKey, user_key.appSecure);

    let client = reqwest::Client::new();
    let resp = client
        .post("https://openapi.youdao.com/api")
        .form(&params)
        .send()
        .await?
        .json::<TransformRes>()
        .await?;

    if resp.errorCode != "0" {
        println!("Error: {:#?}", resp);
    } else {
        // println!("Success: {:#?}", resp);
        display_res(resp);
    }

    Ok(())
}
