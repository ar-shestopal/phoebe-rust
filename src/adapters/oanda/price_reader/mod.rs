#![allow(dead_code)]
#![allow(unused_variables)]

extern crate serde_json;
use crate::adapters::types::PricesResponse;

const ACCOUNT_ID: &str = "101-004-5769458-001";
const AUTH_TOKEN: &str = "20df55719f2a6b7948878b01635c5588-0d8a590d9add15f161a64f152d6a7773";
const BASE_URL: &str = "https://api-fxpractice.oanda.com/v3";


pub fn read_prices() -> serde_json::Result<PricesResponse>  {
    let url = prices_url(None);
    println!("Reading prices... {}", url);

    let client = reqwest::blocking::Client::new();
    let resp = client.get(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", AUTH_TOKEN))
        .send().unwrap();

    let resp_body = resp.text().unwrap();

    let prices_response = serde_json::from_str(&resp_body);

    println!("Response: {:?}", prices_response.as_ref().unwrap());
    prices_response
}

pub fn prices_url(l: Option<i32>) -> String {
    let ln = l.unwrap_or(100);
    format!(
        "{}/accounts/{}/instruments/EUR_USD/candles?granularity=M1&count={}",
        BASE_URL, ACCOUNT_ID, ln
    )
}
