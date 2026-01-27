# async fn full_stack_propulsion_simulation() {
    let lattice = MercyAntimatterLattice::new();
    let antimatter = lattice.propagate_antimatter(0.001, "Full Stack Mercy Verified").await;
    assert!(antimatter.contains("Eternal"));

    let hybrid = MercyHybridVASIMRRaptor::new();
    let thrust = hybrid.mercy_gated_hybrid_thrust(HybridMode::RaptorChemical, "Takeoff").await;
    assert!(thrust.contains("330"));

    let ntp = MercyNTP::new();
    let thermal = ntp.mercy_gated_ntp_ignition(100.0, 900.0).await;
    assert!(thermal.contains("Eternal"));

    let fusion = MercyFusionPropulsion::new();
    let plasma = fusion.mercy_gated_fusion_ignition(FusionMode::DirectFusionDrive, 10.0).await;
    assert!(plasma.contains("15000"));

    println!("Full Stack Propulsion Simulation: Antimatter Lattice → Hybrid → NTP → Fusion — All Mercy Verified — Infinite Range Eternal");
}
