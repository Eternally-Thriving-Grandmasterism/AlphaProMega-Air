//! Hybrid Thrust Test Simulation — MercyHybridVASIMRRaptor Validation
//! Ultramasterful valence-weighted mode transition resonance

use mercy_hybrid_vasimir_raptor::{MercyHybridVASIMRRaptor, HybridMode};

#[tokio::test]
async fn hybrid_thrust_test_simulation() {
    let mut hybrid = MercyHybridVASIMRRaptor::new();

    // Phase 1: Raptor chemical takeoff
    let takeoff = hybrid.mercy_gated_hybrid_thrust(HybridMode::RaptorChemical, "Takeoff Phase Mercy Verified").await;
    assert!(takeoff.contains("330 tons"));

    // Phase 2: Transition to hybrid
    let transition = hybrid.mercy_gated_hybrid_thrust(HybridMode::HybridTransition(0.5), "Transition Phase Mercy Verified").await;
    assert!(transition.contains("Hybrid Transition"));

    // Phase 3: VASIMR plasma cruise
    let cruise = hybrid.mercy_gated_hybrid_thrust(HybridMode::VASIMRPlasma, "Cruise Phase Mercy Verified").await;
    assert!(cruise.contains("5000 s ISP"));

    // Phase 4: Low-valence rejection test
    let reject = hybrid.mercy_gated_hybrid_thrust(HybridMode::RaptorChemical, "Low Valence Attack").await;
    assert!(reject.contains("Mercy Shield"));

    println!("Hybrid Thrust Test Simulation Complete — All Phases Mercy Verified — Infinite Range Eternal");
}
