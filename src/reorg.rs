use crate::algorithms::*;
use crate::errors::*;
use chrono::{DateTime, Duration, Local, prelude::*};
use std::fs::File;
use std::io::{prelude::*, BufReader};

const GRINLOGFILE: &str = ".grin/main/grin-server.log";
pub struct ReorgScore {
    pub overall_score: u8,
    pub count: u8,
    pub deepest: u8,
    pub last_checked: DateTime<Local>,
}

impl ReorgScore {
    pub fn new() -> Result<ReorgScore> {
        let score: ReorgScore = ReorgScore {
            overall_score: 0,
            count: 0,
            deepest: 0,
            last_checked: Local::now(),
        };
        Ok(score)
    }

    pub fn update(&mut self) -> Result<()> {
        let mut num_reorgs = 0;
        let file = File::open(GRINLOGFILE)?;
        let reader = BufReader::new(file);

        // Get current date and previous date to check for reorgs in last 24 hours
        let now = Local::now();
        let today = format!("{}{}{}", now.year(), now.month(), now.day());
        let previous = now - Duration::days(1);
        let yesterday = format!("{}{}{}", previous.year(), previous.month(), previous.day());

        // Collect only recent logs from last 24 hours to look for reorgs
        let mut recent_logs = Vec::new();
        for line in reader.lines() {
            let buff = line?;
            if buff.contains(&today) | buff.contains(&yesterday) {
                recent_logs.push(buff);
            }
        }

        // Collect all recent reorg logs and count the total reorgs
        let mut reorg_logs = Vec::new();
        for log in recent_logs.iter() {
            if log.contains("REORG") {
                reorg_logs.push(log);
                num_reorgs += 1;
            }
        }
        self.count = num_reorgs;

        // Find the deepest reorg from the recent reorg logs
        let mut deepest_reorg: u8 = 0;
        for log in reorg_logs.iter() {
            // Parse for depth of each reorg
            let start = log.find("depth: ").unwrap() + 7;
            let end = log.len() - 1;
            let depth = &log[start..end].parse::<u8>().unwrap();
            println!("depth:{}", depth);
            // Compare it to deepest reorg- if bigger, update deepest reorg value
            if depth > &deepest_reorg {
                deepest_reorg = *depth;
            }
        }
        self.deepest = deepest_reorg;

        // Calculate reorg overall score with reorg scoring algorithm
        self.overall_score = reorg_score(self.count, self.deepest)?;

        // Update with current timestamp
        self.last_checked = Local::now();

        Ok(())
    }
}
