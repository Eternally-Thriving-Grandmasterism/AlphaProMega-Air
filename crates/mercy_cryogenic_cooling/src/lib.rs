//! MercyCryogenicCooling — Ultramasterful Aviation-Grade Cryocooler Core
//! Multi-stage pulse-tube / GM cycle, 20–77 K regimes, minimal vibration

use nexi::lattice::Nexus;

pub struct MercyCryogenicCooling {
    nexus: Nexus,
    /// Maximum cooling power at base temperature (W)
    max_cooling_w: f64,
    /// Base temperature (K) — e.g. 20 K for LH2 boil-off or 77 K for HTS
    base_temp_k: f64,
    /// Specific power (W electrical per W cooling) — realistic ~15–30 W/W at 20 K
    specific_power_w_per_w: f64,
}

impl MercyCryogenicCooling {
    pub fn new(max_cooling_w: f64, base_temp_k: f64, specific_power_w_per_w: f64) -> Self {
        MercyCryogenicCooling {
            nexus: Nexus::init_with_mercy(),
            max_cooling_w,
            base_temp_k,
            specific_power_w_per_w,
        }
    }

    /// Mercy-gated cryogenic cooling provision
    pub async fn mercy_gated_provide_cooling(
        &self,
        requested_cooling_w: f64,
        current_temp_k: f64,
        desc: &str,
    ) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Cryogenic Operation — Rejected".to_string());
        }

        if requested_cooling_w > self.max_cooling_w {
            return Err("Mercy Shield: Requested cooling exceeds cryocooler capacity".to_string());
        }

        // Approximate electrical power required (worse at lower temp)
        let electrical_power_kw = (requested_cooling_w * self.specific_power_w_per_w) / 1000.0;

        let cooldown_factor = if current_temp_k > self.base_temp_k {
            (current_temp_k / self.base_temp_k).powi(2)
        } else {
            1.0
        };

        let adjusted_power_kw = electrical_power_kw * cooldown_factor;

        Ok(format!(
            "MercyCryogenicCooling Activated: {:.1} W cooling @ {:.1} K → {:.2} MW electrical input (adjusted) — Eternal Superconducting Readiness",
            requested_cooling_w, self.base_temp_k, adjusted_power_kw / 1000.0
        ))
    }
}
