// api.rs

use reqwest;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub async fn fetch_stock_data(symbol: &str) -> Result<Value, Box<dyn Error>> {
    let url = "https://alpha-vantage.p.rapidapi.com/query";

    let mut querystring = HashMap::new();
    querystring.insert("interval", "5min");
    querystring.insert("function", "TIME_SERIES_INTRADAY");
    querystring.insert("symbol", symbol);
    querystring.insert("datatype", "json");
    querystring.insert("output_size", "compact");

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("X-RapidAPI-Key", "your_api_key_here") // Replace with your API key
        .header("X-RapidAPI-Host", "alpha-vantage.p.rapidapi.com")
        .query(&querystring)
        .send()
        .await?;

    let json = response.json::<Value>().await?;
    Ok(json)
}
