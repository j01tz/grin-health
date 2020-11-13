use chrono::{DateTime, Local};

use crate::algorithms::*;
use crate::api::*;
use crate::errors::*;

// API endpoints for gathering necessary data for nicehash score
const CURRENT: &str = "https://api2.nicehash.com/main/api/v2/public/stats/global/current";
const AVERAGE: &str = "https://api2.nicehash.com/main/api/v2/public/stats/global/24h";
const NETSPEED: &str = "https://api.grinmint.com/v2/networkStats";
const BTCGRIN: &str = "https://api.coingecko.com/api/v3/simple/price?ids=grin&vs_currencies=btc";

// Contains relevant Nicehash score data to calculate overall health score
#[derive(Debug)]
pub struct NicehashScore {
    pub overall_score: u8,
    pub current_price: f64,
    pub average_price: f64,
    pub current_speed: f64,
    pub average_speed: f64,
    pub last_checked: DateTime<Local>,
}

impl NicehashScore {
    // Creates a new NicehashScore struct to store values
    pub fn new() -> Result<NicehashScore> {
        let score: NicehashScore = NicehashScore {
            overall_score: 0,
            current_price: 0.0,
            average_price: 0.0,
            current_speed: 0.0,
            average_speed: 0.0,
            last_checked: Local::now(),
        };
        Ok(score)
    }

    // Updates a NicehashScore to reflect current conditions
    pub fn update(&mut self) -> Result<()> {
        // Get current price and speed as [price, speed]
        let current_data: Vec<f64> = parse_nicehash(call(CURRENT)?)?;
        self.current_price = current_data[0];
        self.current_speed = current_data[1];

        // Get average price and speed as [price, speed]
        let average_data: Vec<f64> = parse_nicehash(call(AVERAGE)?)?;
        self.average_price = average_data[0];
        self.average_speed = average_data[1];

        // Get current network speed
        let network_speed: f64 = parse_net_speed(call(NETSPEED)?)?;

        // Get current BTC/GRIN ratio
        let btc_grin_ratio: f64 = parse_btc_grin(call(BTCGRIN)?)?;

        // Calculate nicehash overall score with nicehash scoring algorithm
        self.overall_score = nicehash_score(
            self.current_price,
            self.current_speed,
            self.average_price,
            self.average_speed,
            network_speed,
            btc_grin_ratio,
        )?;

        // Update with current timestamp
        self.last_checked = Local::now();

        Ok(())
    }
}
