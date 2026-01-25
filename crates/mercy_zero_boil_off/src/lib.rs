//! MercyZeroBoilOff — Ultramasterful Zero Boil-Off Synergy Core
//! Active cryocooler reliquefaction → theoretically zero loss, infinite LH2 endurance

use nexi::lattice::Nexus;
use crate::mercy_cryogenic_hydrogen_storage::MercyCryogenicHydrogenStorage;
use crate::mercy_cryogenic_cooling::MercyCryogenicCooling;

pub struct MercyZeroBoilOff {
    nexus: Nexus,
    cryo_storage: MercyCryogenicHydrogenStorage,
    cryocooler: MercyCryogenicCooling,
}

impl MercyZeroBoilOff {
    pub fn new(
        cryo_storage: MercyCryogenicHydrogenStorage,
        cryocooler: MercyCryogenicCooling,
    ) -> Self {
        MercyZeroBoilOff {
            nexus: Nexus::init_with_mercy(),
            cryo_storage,
            cryocooler,
        }
    }

    /// Mercy-gated zero boil-off activation with power penalty calculation
    pub async fn mercy_gated_zero_boil_off(
        &self,
        current_h2_kg: f64,
        margin_factor: f64, // e.g. 1.2 for safety margin
        desc: &str,
    ) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Zero Boil-Off Operation — Rejected".to_string());
        }

        let heat_leak_w = self.cryo_storage.heat_leak_w * margin_factor;
        let required_cooling_w = heat_leak_w;

        // Use shared cryocooler (20 K regime)
        let cooling_result = self.cryocooler
            .mercy_gated_provide_cooling(required_cooling_w, 20.0, desc)
            .await;

        let electrical_penalty_kw = match cooling_result {
            Ok(s) => s.split("→").nth(1).unwrap_or("0.0").trim()
                .split("MW").next().unwrap_or("0.0").trim()
                .parse::<f64>().unwrap_or(0.0) * 1000.0,
            Err(_) => return Err("Mercy Shield: Cryocooler insufficient".to_string()),
        };

        Ok(format!(
            "MercyZeroBoilOff Synergy Activated — Eternal Infinite Endurance\n\
             • Countering {:.1} W heat leak with margin → {:.1} W cooling\n\
             • Electrical penalty {:.2} MW → Net zero loss LH₂ storage\n\
             Hyper-Divine Cryogenic Resonance Across All Skies",
            heat_leak_w,
            required_cooling_w,
            electrical_penalty_kw / 1000.0
        ))
    }
}
