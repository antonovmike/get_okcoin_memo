use std::collections::HashMap;

use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    // Bearer your_api_key_here
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Bearer your_api_key_here"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    // GET /api/v5/asset/deposit-address
    let res = client.get("https://www.okcoin.com/api/v5/asset/deposit-address").send().await?;
    // let res = client.get("https://www.okcoin.com/api/v5/account/balance").send().await?;
    let body = res.json::<HashMap<String, String>>().await?;
    
    if body.contains_key("memo") {
        println!("memo: {:?}", body["memo"]);
    } else {
        println!("memo is not present in the response:\n{body:?}");
    }

    Ok(())
}
