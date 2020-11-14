use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::errors::*;
use crate::nicehash::*;
use crate::reorg::*;
use crate::algorithms::health_score;

#[macro_use]
extern crate error_chain;

pub mod nicehash;
pub mod reorg;
pub mod algorithms;
mod errors;
mod api;

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthData {
    nicehash_data: NicehashScore,
    reorg_data: ReorgScore,
}

impl HealthData {
    pub fn new() -> Result<HealthData> {
        let data: HealthData = HealthData {
            nicehash_data: NicehashScore::new()?,
            reorg_data: ReorgScore::new()?,
        };
        Ok(data)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthScore {
    pub overall_score: u8,
    pub nicehash_score: u8,
    pub reorg_score: u8,
    pub last_checked: DateTime<Utc>,
    pub data: HealthData,
}

impl HealthScore {
    pub fn new() -> Result<HealthScore> {
        let score: HealthScore = HealthScore {
            overall_score: 0,
            nicehash_score: 0,
            reorg_score: 0,
            last_checked: Utc::now(),
            data: HealthData::new()?,
        };
        Ok(score)
    }

    pub fn update(&mut self) -> Result<()> {
        // Generate a new nicehash score
        let mut nicehash: NicehashScore = NicehashScore::new()?;
        NicehashScore::update(&mut nicehash)?;
        self.nicehash_score = nicehash.overall_score;

        // Generate a new reorg score
        let mut reorg: ReorgScore = ReorgScore::new()?;
        ReorgScore::update(&mut reorg)?;
        self.reorg_score = reorg.overall_score;

        // Calculate GRIN network overall health score with scoring algorithm
        self.overall_score = health_score(nicehash.overall_score, reorg.overall_score)?;

        // Update with current timestamp
        self.last_checked = Utc::now();

        // Include granular nicehash and reorg data points
        self.data.nicehash_data = nicehash;
        self.data.reorg_data = reorg;

        Ok(())
    }
}