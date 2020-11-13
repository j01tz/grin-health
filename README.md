# grin-health
Library that provides a health score and health data for the Grin network

WARNING: these scores may be innacurate. You should not rely on them yet to assess payment finality for Grin.

Quickstart:
```
let mut health = grin_health::HealthScore::new().unwrap();
grin_health::HealthScore::update(&mut health).unwrap();
println!("{:?}", health);
```