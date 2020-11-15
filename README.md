# grin-health

`grin-health` is a library that provides a health score and health data for the Grin network.

WARNING: these scores may be innacurate. You should not blindly rely on them to assess payment finality for Grin.

## Summary

This library uses data from the Nicehash API and from local Grin node logs to determine an overall health score for the Grin network that can be used to measure real-time risk for accepting payments.

### Requirements

- Grin node running locally and producing logs
    - Adjust log path with `GRINLOGFILE` in `reorg.rs`

- Ability to make external API calls

### Quickstart

```
grin-health = { git = "https://github.com/j01tz/grin-health", branch = "main" }
```

```
use grin_health;

fn main() {
    let mut health = grin_health::HealthScore::new().unwrap();
    grin_health::HealthScore::update(&mut health).unwrap();
    println!("{:?}", health);
}
```

## Scores

Scoring is rated 0 to 5, with 5 being the healthiest network and 0 being a network likely currently under attack and risky to use to accept payments. Scores in between can help adjust confirmation times to manage risk in real-time. 

The suggestions below are just suggestions- you must use your own judgment when determining confirmation times for your own use case.

```
Suggested confirmation times for scores:
0 = Do not accept payments
1 = 10080 confirmations 
2 = 1440 confirmations
3 = 180 confirmations
4 = 60 confirmations
5 = 30 confirmations
```

## Algorithms

Scoring algorithms are used to determine the health status. They will be updated and improved as more data becomes available.

The following algorithms are used to calculate the overall health score:

### Nicehash

This algorithm consumes the following data: current rental price/graph/day, average rental price/graph/day, current nicehash speed, average nicehash speed, overall network speed and the current BTC/GRIN price ratio.

Based on this data, the scoring algorithm attempts to generate a 0 to 5 score to accurately reflect risk posed to the Grin network from current Nicehash dynamics.

### Reorg

This algorithm consumes the following data: number of reorganizations in the last day and highest depth of reorganization in the last day.

A high volume or depth of block reorganizations on the network can indicate an attack or poor network health. Based on this data the scoring algorithm attempts to generate a 0 to 5 score to accurately reflect risk posed to the Grin network from current network reorganization history.

A live Grin node is required to run to produce the logs to be consumed for reorganization data. In the future this data may be available via external API.

### Overall Health

This algorithm consumes the nicehash and reorg scores to generate a 0 to 5 score to attempt to accurately reflect the overall network health for Grin in real-time.