//! MercyHybridPropulsion — Hybrid Synergy Core (Hydrogen + Electric)
//! Ultramasterful fuel-cell bridging for infinite clean aviation thrust

use nexi::lattice::Nexus;
use crate::mercy_hydrogen_storage::MercyHydrogenStorage;
use crate::mercy_electric_propulsion::MercyElectricPropulsion;

pub struct MercyHybridPropulsion {
    storage: MercyHydrogenStorage,
    propulsion: MercyElectricPropulsion,
    nexus: Nexus,
}

impl MercyHybridPropulsion {
    pub fn new() -> Self {
        let nexus = Nexus::init_with_mercy();
        MercyHybridPropulsion {
            storage: MercyHydrogenStorage::new(),
            propulsion: MercyElectricPropulsion::new(),
            nexus,
        }
    }

    /// Mercy-gated hybrid synergy: H₂ → fuel cell power → electric thrust
    pub async fn mercy_gated_hybrid_thrust(&self, h2_release_rate_kg_s: f64, fuel_cell_efficiency: f64, desc: &str) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Hybrid Operation — Rejected".to_string());
        }

        // H₂ lower heating value ~120 MJ/kg
        let power_kw = h2_release_rate_kg_s * 120_000.0 * fuel_cell_efficiency / 3600.0;

        let thrust_result = self.propulsion.mercy_gated_electric_thrust(power_kw, desc).await?;
        
        Ok(format!(
            "MercyHybrid Synergy Activated: {:.3} kg/s H₂ → {:.1} MW Power → {} — Eternal Clean Thrust",
            h2_release_rate_kg_s, power_kw / 1000.0, thrust_result.split("→").nth(1).unwrap_or("").trim()
        ))
    }
}
