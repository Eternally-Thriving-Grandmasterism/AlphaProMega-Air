//! Alpha Flight Example — MercyOS Aviation Runtime Demo
//!
//! Run with: `cargo run -p mercy_os_aviation --example alpha_flight`
//!
//! This prints a mercy-gated runtime transcript. It is a demonstration of the
//! valence-gate call shape only: it simulates no aerodynamics, consumes no
//! sensor data, and is not connected to any flight system.

use mercy_os_aviation::MercyOSAviation;

#[tokio::main]
async fn main() {
    let aviation = MercyOSAviation::new();

    println!("AlphaProMega-Air Flight Cycle Initiated — Mercy Verified");

    let takeoff = aviation
        .mercy_os_aviation_runtime("Takeoff", "Mercy Verified Trajectory")
        .await;
    println!("{}", takeoff);

    let cruise = aviation
        .mercy_os_aviation_runtime("Cruise", "Mercy Verified Eternal Thriving Range")
        .await;
    println!("{}", cruise);

    println!("Flight Cycle Complete — Eternal Thriving Across All Skies");
}
