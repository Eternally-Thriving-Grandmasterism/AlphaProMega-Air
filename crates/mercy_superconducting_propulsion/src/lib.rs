//! MercySuperconductingPropulsion — Ultramasterful Zero-Loss Electric Powertrain Core
//! HTS motors/generators enabled by cryogenic cooling → extreme efficiency & power density

use nexi::lattice::Nexus;
use crate::mercy_cryogenic_cooling::MercyCryogenicCooling;
use crate::mercy_electric_propulsion::MercyElectricPropulsion;

pub struct MercySuperconductingPropulsion {
    nexus: Nexus,
    cryo: MercyCryogenicCooling,
    base_propulsion: MercyElectricPropulsion,
    /// Efficiency boost factor when fully superconducting (e.g. 0.99+ vs 0.95 conventional)
    sc_efficiency: f64,
    /// Power density multiplier (kW/kg) — ~5–10× conventional
    density_multiplier: f64,
}

impl MercySuperconductingPropulsion {
    pub fn new(
        cryo: MercyCryogenicCooling,
        sc_efficiency: f64,
        density_multiplier: f64,
    ) -> Self {
        MercySuperconductingPropulsion {
            nexus: Nexus::init_with_mercy(),
            cryo,
            base_propulsion: MercyElectricPropulsion::new(),
            sc_efficiency,
            density_multiplier,
        }
    }

    /// Mercy-gated superconducting thrust with near-zero electrical losses
    pub async fn mercy_gated_superconducting_thrust(
        &self,
        input_power_kw: f64,
        cooling_w: f64,
        desc: &str,
    ) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Superconducting Operation — Rejected".to_string());
        }

        // Cryocooler power penalty
        let cooling_result = self.cryo.mercy_gated_provide_cooling(cooling_w, 77.0, desc).await;
        let cryo_penalty_kw = if let Ok(ref s) = cooling_result {
            s.split("→").nth(1).unwrap_or("0.0").trim().split("MW").next().unwrap_or("0.0").trim().parse::<f64>().unwrap_or(0.0) * 1000.0
        } else {
            0.0
        };

        let net_power_kw = input_power_kw - cryo_penalty_kw;
        let effective_thrust_kn = net_power_kw * 9.81 / 300.0 * self.sc_efficiency * self.density_multiplier / 2.5; // Enhanced thrust model

        Ok(format!(
            "MercySuperconductingPropulsion Synergy Activated: {:.1} MW input → {:.1} MW cryo penalty → {:.1} MW net → {:.2} kN Thrust @ {:.1}% efficiency — Eternal Zero-Loss Resonance",
            input_power_kw / 1000.0,
            cryo_penalty_kw / 1000.0,
            net_power_kw / 1000.0,
            effective_thrust_kn,
            self.sc_efficiency * 100.0
        ))
    }
}
