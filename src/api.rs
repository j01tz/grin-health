use serde_json::Value;
use crate::errors::*;

// Returns responses from a basic GET request to external API as json values
#[tokio::main]
pub async fn call(url: &str) -> Result<Value> {
    let res = reqwest::get(url).await?.json().await?;
    Ok(res)
}

// Parses nicehash json data to return [price, speed] for GRIN
pub fn parse_nicehash(data: Value) -> Result<Vec<f64>> {
    let mut new_data: Vec<f64> = Vec::new();
    let algos = data["algos"].as_array();
    match algos {
        Some(algos) => {
            for a in algos {
                // GRIN is assigned the value 50 by nicehash as an identifier
                if a["a"] == 50 {
                    new_data.push(a["p"].as_f64().ok_or_else(|| "Invalid nicehash data!")?);
                    new_data.push(a["s"].as_f64().ok_or_else(|| "Invalid nicehash data!")?);
                }
            }
        },
        None => {
            return Err("Invalid nicehash data!".into())
        }
    };
    Ok(new_data)
}

// Parses grinmint data to derive current network graphrate
pub fn parse_net_speed(data: Value) -> Result<f64> {
    Ok(data["hashrates"]["32"].as_f64().ok_or_else(|| "Invalid grinmint data!")?)
}

// Parses coingecko data to derive current BTC/GRIN ratio
pub fn parse_btc_grin(data: Value) -> Result<f64> {
    Ok(data["grin"]["btc"].as_f64().ok_or_else(|| "Invalid grinmint data!")?)
}