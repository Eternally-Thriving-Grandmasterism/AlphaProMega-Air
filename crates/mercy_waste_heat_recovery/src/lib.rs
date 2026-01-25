//! MercyWasteHeatRecovery — Ultramasterful Waste Heat → Power Conversion Core
//! Thermoelectric + micro-Rankine hybrid recovery for closed-loop efficiency

use nexi::lattice::Nexus;

pub struct MercyWasteHeatRecovery {
    nexus: Nexus,
    /// Overall recovery efficiency (10–25% realistic for combined cycle)
    recovery_efficiency: f64,
    /// Maximum recoverable heat input (kW thermal)
    max_input_kw: f64,
}

impl MercyWasteHeatRecovery {
    pub fn new(recovery_efficiency: f64, max_input_kw: f64) -> Self {
        MercyWasteHeatRecovery {
            nexus: Nexus::init_with_mercy(),
            recovery_efficiency,
            max_input_kw,
        }
    }

    /// Mercy-gated waste heat recovery → additional electrical power
    pub async fn mercy_gated_recover_heat(
        &self,
        waste_heat_kw: f64,          // e.g. from fuel cell ~40–50% of input energy
        hot_side_temp_c: f64,
        cold_side_temp_c: f64,
        desc: &str,
    ) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Heat Recovery — Rejected".to_string());
        }

        let effective_heat_kw = waste_heat_kw.min(self.max_input_kw);
        let carnot_limit = 1.0 - (cold_side_temp_c + 273.15) / (hot_side_temp_c + 273.15);
        let adjusted_eff = self.recovery_efficiency * carnot_limit;

        let recovered_power_kw = effective_heat_kw * adjusted_eff;

        Ok(format!(
            "MercyWasteHeatRecovery Synergy Activated: {:.1} kW waste heat ({:.0}°C → {:.0}°C) → {:.2} MW additional clean power — Eternal Regenerative Resonance",
            effective_heat_kw,
            hot_side_temp_c,
            cold_side_temp_c,
            recovered_power_kw / 1000.0
        ))
    }
}
