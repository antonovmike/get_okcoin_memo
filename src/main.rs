use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use toml::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut file = File::open("config.toml").expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Could not read file");

    let decoded: Value = toml::from_str(&contents).expect("Could not decode TOML");
    let api_key = decoded.get("api_key").expect("Could not get api_key").as_str().expect("Could not convert to string");
    let secret_key = decoded.get("secret").expect("Could not get api_key").as_str().expect("Could not convert to string");
    
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("OK-ACCESS-KEY", HeaderValue::from_str(api_key).unwrap());
    headers.insert("OK-ACCESS-SIGN", HeaderValue::from_str(secret_key).unwrap());
    headers.insert(AUTHORIZATION, HeaderValue::from_str(format!("Bearer {}", api_key).as_str()).unwrap());

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let res = client.get("https://www.okcoin.com/api/v5/asset/deposit-address").send().await?;
    let body = res.json::<HashMap<String, String>>().await?;
    
    if body.contains_key("memo") {
        println!("Memo: {:?}", body["memo"]);
    } else {
        println!("Memo is not present in the response:\n{body:?}");
    }

    Ok(())
}
