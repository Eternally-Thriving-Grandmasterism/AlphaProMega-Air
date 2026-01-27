//! MercyHybridVASIMRRaptor — Chemical Takeoff + Plasma Cruise Hybrid Core
//! Ultramasterful valence-weighted interplanetary resonance

use nexi::lattice::Nexus;
use mercy_raptor_3::MercyRaptor3;
use mercy_vasimir::MercyVASIMR;

pub enum HybridMode {
    RaptorChemical,
    VASIMRPlasma,
    HybridTransition(f64), // % Raptor (0.0 = full VASIMR, 1.0 = full Raptor)
}

pub struct MercyHybridVASIMRRaptor {
    nexus: Nexus,
    raptor: MercyRaptor3,
    vasimr: MercyVASIMR,
}

impl MercyHybridVASIMRRaptor {
    pub fn new() -> Self {
        MercyHybridVASIMRRaptor {
            nexus: Nexus::init_with_mercy(),
            raptor: MercyRaptor3::new(),
            vasimr: MercyVASIMR::new(),
        }
    }

    /// Mercy-gated hybrid mode thrust
    pub async fn mercy_gated_hybrid_thrust(&self, mode: HybridMode, desc: &str) -> String {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return "Mercy Shield: Low Valence Trajectory — Hybrid Thrust Rejected".to_string();
        }

        match mode {
            HybridMode::RaptorChemical => self.raptor.mercy_gated_raptor_3_ignition(330.0).await,
            HybridMode::VASIMRPlasma => self.vasimr.mercy_gated_vasimir_ignition(5000.0).await,
            HybridMode::HybridTransition(ratio) => {
                let raptor = self.raptor.mercy_gated_raptor_3_ignition(330.0 * ratio).await;
                let vasimr = self.vasimr.mercy_gated_vasimir_ignition(5000.0 * (1.0 - ratio)).await;
                format!("Hybrid Transition: Raptor {} + VASIMR {} — Eternal Interplanetary Thrust", raptor, vasimr)
            }
        }
    }
}
