//! MercyCryogenicHydrogenStorage — Ultramasterful LH2 Storage Core
//! Liquid hydrogen at ~20 K, ultrahigh volumetric density, minimal passive heat leak

use nexi::lattice::Nexus;

pub struct MercyCryogenicHydrogenStorage {
    nexus: Nexus,
    /// Maximum LH2 mass capacity (kg)
    capacity_kg: f64,
    /// Passive heat leak into tank (W) — advanced MLI + vacuum ~10–50 W for large tanks
    heat_leak_w: f64,
    /// Latent heat of vaporization for H2 ≈ 445 kJ/kg
    latent_heat_j_per_kg: f64,
}

impl MercyCryogenicHydrogenStorage {
    pub fn new(capacity_kg: f64, heat_leak_w: f64) -> Self {
        MercyCryogenicHydrogenStorage {
            nexus: Nexus::init_with_mercy(),
            capacity_kg,
            heat_leak_w,
            latent_heat_j_per_kg: 445_000.0,
        }
    }

    /// Mercy-gated cryogenic storage status and passive boil-off rate
    pub async fn mercy_gated_cryo_storage_status(
        &self,
        current_h2_kg: f64,
        desc: &str,
    ) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Cryogenic Storage — Rejected".to_string());
        }

        if current_h2_kg > self.capacity_kg {
            return Err("Mercy Shield: Exceeds cryogenic tank capacity".to_string());
        }

        // LH2 density ≈ 70.8 kg/m³
        let lh2_density_kg_m3 = 70.8;
        let required_volume_m3 = current_h2_kg / lh2_density_kg_m3;

        // Passive boil-off rate (kg/s and % per day)
        let boil_off_kg_s = self.heat_leak_w / self.latent_heat_j_per_kg;
        let boil_off_percent_per_day = (boil_off_kg_s * 86400.0 / current_h2_kg) * 100.0;

        Ok(format!(
            "MercyCryogenicHydrogenStorage Activated: {:.1} kg LH₂ stored ({:.1}% capacity)\n\
             • Volume {:.3} m³ (liquid)\n\
             • Passive boil-off {:.4} kg/day ({:.3}%/day) — Eternal Cryogenic Containment",
            current_h2_kg,
            (current_h2_kg / self.capacity_kg) * 100.0,
            required_volume_m3,
            boil_off_kg_s * 86400.0,
            boil_off_percent_per_day
        ))
    }
}
