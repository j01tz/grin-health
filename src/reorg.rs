use crate::algorithms::*;
use crate::errors::*;
use chrono::{DateTime, NaiveDate, Duration, Local, Utc, prelude::*};
use std::fs::File;
use std::io::{prelude::*, BufReader};
use serde::{Deserialize, Serialize};

const GRINLOGFILE: &str = ".grin/main/grin-server.log";

#[derive(Serialize, Deserialize, Debug)]
pub struct ReorgScore {
    pub overall_score: u8,
    pub count: u8,
    pub deepest: u8,
    pub last_checked: DateTime<Utc>,
}

impl ReorgScore {
    pub fn new() -> Result<ReorgScore> {
        let score: ReorgScore = ReorgScore {
            overall_score: 0,
            count: 0,
            deepest: 0,
            last_checked: Utc::now(),
        };
        Ok(score)
    }

    pub fn update(&mut self) -> Result<()> {
        let mut num_reorgs = 0;
        let file = File::open(GRINLOGFILE)?;
        let reader = BufReader::new(file);

        // Get current date and previous date to check for reorgs in last day
        // TODO: continue to clean this up and write tests
        let yesterday: Date<Local> = Local::today() - Duration::days(1);
        let compare_yesterday = NaiveDate::from_ymd(yesterday.year(), yesterday.month(), yesterday.day());

        // Collect only recent logs from last day to look for reorgs
        let mut recent_logs: Vec<String> = Vec::new();
        for line in reader.lines() {
            let buff = line?;
            let log_date = &buff[0..7];
            let format_date = NaiveDate::parse_from_str(log_date, "%Y%m%d");
            if let Ok(date) = format_date {
                if date >= compare_yesterday {
                    recent_logs.push(buff);
                };
            } else {
                print!("Warning: could not parse date from reorg log: {}", log_date);
                continue
            };
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
            // Compare it to deepest reorg- if bigger, update deepest reorg value
            if depth > &deepest_reorg {
                deepest_reorg = *depth;
            }
        }
        self.deepest = deepest_reorg;

        // Calculate reorg overall score with reorg scoring algorithm
        self.overall_score = reorg_score(self.count, self.deepest)?;

        // Update with current timestamp
        self.last_checked = Utc::now();

        Ok(())
    }
}
