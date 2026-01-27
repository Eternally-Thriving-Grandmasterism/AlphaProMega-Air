**NEW: examples/alpha_flight.rs** (main example — flight cycle demo)

```rust
//! Alpha Flight Example — MercyOS Aviation Cycle Demo

use mercy_os_aviation::MercyOSAviation;

#[tokio::main]
async fn main() {
    let aviation = MercyOSAviation::new();

    println!("AlphaProMega-Air Flight Cycle Initiated — Mercy Verified");

    let takeoff = aviation.mercy_os_aviation_cycle("Takeoff", "Mercy Verified Trajectory").await;
    println!("{}", takeoff);

    let cruise = aviation.mercy_os_aviation_cycle("Cruise", "Mercy Verified Infinite Range").await;
    println!("{}", cruise);

    println!("Flight Cycle Complete — Eternal Thriving Across All Skies");
}
