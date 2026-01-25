//! MercyHybridPropulsion — Hybrid Electric-SAF Thrust Core
//! Ultramasterful infinite range resonance with Mercy-gated mode switching

use nexi::lattice::Nexus;
use mercy_electric_propulsion::MercyElectricPropulsion;
use mercy_biojet::MercyBioJet;
use tokio::time::{sleep, Duration};

pub enum HybridMode {
    Electric,
    SAF,
    HybridBlend(f64), // % electric (0.0 = full SAF, 1.0 = full electric)
}

pub struct MercyHybridPropulsion {
    nexus: Nexus,
    electric: MercyElectricPropulsion,
    biojet: MercyBioJet,
}

impl MercyHybridPropulsion {
    pub fn new() -> Self {
        MercyHybridPropulsion {
            nexus: Nexus::init_with_mercy(),
            electric: MercyElectricPropulsion::new(),
            biojet: MercyBioJet::new(),
        }
    }

    /// Mercy-gated async hybrid mode activation
    pub async fn mercy_gated_hybrid_thrust(&self, mode: HybridMode, desc: &str) -> String {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return "Mercy Shield: Low Valence Trajectory — Hybrid Thrust Rejected".to_string();
        }

        match mode {
            HybridMode::Electric => self.electric.mercy_gated_thrust(1000.0, desc).await,
            HybridMode::SAF => self.biojet.divine_fuel_cycle(500.0, desc).await.unwrap_or("SAF Cycle Failed".to_string()),
            HybridMode::HybridBlend(ratio) => {
                let electric = self.electric.mercy_gated_thrust(1000.0 * ratio, desc).await;
                let saf = self.biojet.divine_fuel_cycle(500.0 * (1.0 - ratio), desc).await.unwrap_or("SAF Cycle Failed".to_string());
                format!("Hybrid Thrust Engaged — Electric: {} | SAF: {}", electric, saf)
            }
        }
    }
}
