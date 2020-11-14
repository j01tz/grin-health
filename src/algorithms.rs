use crate::errors::*;

// Calculates an overall health score based on nicehash and reorg data
pub fn health_score(nicehash_score: u8, reorg_score: u8) -> Result<u8> {
    let score: u8;
    if reorg_score == 0 && nicehash_score == 0 {
        score = 0;
        return Ok(score);
    };

    score = (nicehash_score + reorg_score) / 2;

    Ok(score)
}

// Calculates a score based on nicehash, network and pricing data
// Values must use same decimal placement provided by nicehash/grinmint/coingecko APIs
pub fn nicehash_score(
    current_price: f64,
    current_speed: f64,
    average_price: f64,
    average_speed: f64,
    network_speed: f64,
    btc_grin_ratio: f64,
) -> Result<u8> {
    let mut score: u8 = 5;

    // Calculate BTC earned for each graph/day
    // 60 grin/block * 60 blocks/hour * 24 hours/day = 86400
    // Multiply to make decimal match with speed and price values
    let earning_per_graph = ((btc_grin_ratio * 86400.0) / network_speed) * 100000000.0;

    // Calculate ratios to use for scoring algorithm
    let profitability_ratio: f64 = current_price / earning_per_graph;
    let price_ratio: f64 = current_price / average_price;
    let speed_ratio: f64 = current_speed / average_speed;
    let network_ratio: f64 = current_speed / network_speed;

    // Begin scoring alogirthm:
    // Point lost if current market price is 50% or more of earning price
    // Compares current nicehash price/graph with current value of each graph on the network
    if profitability_ratio >= 1.5 {
        score -= 1;
    }

    // Point lost if current price has increased by 50% or more compared to daily average
    if price_ratio >= 1.5 {
        score -= 1;
    }

    // Another point lost if the current price has doubled compared to daily average
    if price_ratio >= 2.0 {
        score -= 1;
    }

    // Point lost if current nicehash graphrate is 50% or more of GRIN network
    if network_ratio >= 0.5 {
        score -= 1;
    }

    // Another point lost of the current nicehash graphrate is 75% or more of GRIN network
    if network_ratio >= 0.75 {
        score -= 1;
    }

    // If we aren't already at score 1, remove a point if NH hash >90% network graphrate
    if score > 1 && network_ratio >= 0.9 {
        score -= 1;
    }

    // See if the current nicehash speed has doubled from average
    // If there are still points we can take away, remove 1 from score
    if score > 1 && speed_ratio >= 1.5 {
        score -= 1;
    }

    // Even if there is no major changes on speed and price, if nicehash is responsible
    // for 50% or more of the network graphrate, the network is still fairly unhealthy.
    if score == 4 && network_ratio >= 0.5 {
        score -= 1;
    }

    Ok(score)
}

// Calculates a score based on recent REORG data
pub fn reorg_score(count: u8, deepest: u8) -> Result<u8> {
    let mut reorg_score: u8 = 5;

    // Begin scoring algorithm:
    // Point lost if daily reorg count exceeds 5 and depth exceed 1
    // This value chosen based on 2 months of node logs
    // May need to be adjusted as more data is collected
    if count >= 5 && deepest > 1 {
        reorg_score -= 1;
    }

    // More than 20 reorgs daily has not yet been seen but may occur during extended attack
    if count > 20 {
        reorg_score -= 1;
    }

    // Reorgs as deep as 5 are concerning as unlikely accidental
    if deepest >= 5 {
        reorg_score -= 1;
    }

    // 5+ depth reorgs are likely intentional
    // This will impact accepting low-conf payments
    if deepest >= 5 {
        reorg_score -= 1;
    }

    // Attacked with intention reorgs at 15+
    // Mitigatable with reasonable confirmation times
    if deepest >= 15 {
        reorg_score = 2;
    }

    // 30+ deep reorgs are severely disruptive
    if deepest >= 30 {
        reorg_score = 1;
    }

    // 60+ deep reorgs mean the network is under heavy attack
    // Advise to wait days for large payment finality
    if deepest >= 60 {
        reorg_score = 0;
    }

    Ok(reorg_score)
}
