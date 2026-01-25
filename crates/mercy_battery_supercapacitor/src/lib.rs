//! MercyBatterySupercapacitor — Ultramasterful Hybrid Energy Storage Core
//! High energy density (battery) + extreme power density (supercapacitor) resonance

use nexi::lattice::Nexus;

pub struct MercyBatterySupercapacitor {
    nexus: Nexus,
    /// Total battery energy capacity (kWh) — advanced solid-state ~1000 Wh/kg placeholder
    battery_capacity_kwh: f64,
    /// Max continuous discharge power from battery (kW)
    battery_max_power_kw: f64,
    /// Small but ultra-fast supercapacitor energy capacity (kWh)
    supercap_capacity_kwh: f64,
    /// Max peak power from supercapacitor (kW)
    supercap_max_power_kw: f64,
}

impl MercyBatterySupercapacitor {
    pub fn new(
        battery_capacity_kwh: f64,
        battery_max_power_kw: f64,
        supercap_capacity_kwh: f64,
        supercap_max_power_kw: f64,
    ) -> Self {
        MercyBatterySupercapacitor {
            nexus: Nexus::init_with_mercy(),
            battery_capacity_kwh,
            battery_max_power_kw,
            supercap_capacity_kwh,
            supercap_max_power_kw,
        }
    }

    /// Mercy-gated hybrid discharge profile at requested power (assumes full charge)
    pub async fn mercy_gated_hybrid_discharge(
        &self,
        requested_power_kw: f64,
        desc: &str,
    ) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Discharge — Rejected".to_string());
        }

        let base_power_kw = self.battery_max_power_kw;
        let peak_power_kw = base_power_kw + self.supercap_max_power_kw;

        let mut profile = format!(
            "MercyBatterySupercapacitor Hybrid Activated: Requested {:.1} MW\n",
            requested_power_kw / 1000.0
        );

        if requested_power_kw <= base_power_kw {
            let sustained_hours = self.battery_capacity_kwh / requested_power_kw;
            profile += &format!(
                "→ Sustained {:.2} hours @ {:.1} MW (battery only) — Eternal Endurance",
                sustained_hours, requested_power_kw / 1000.0
            );
        } else if requested_power_kw <= peak_power_kw {
            let excess_kw = requested_power_kw - base_power_kw;
            let burst_hours = self.supercap_capacity_kwh / excess_kw;
            let sustained_hours = self.battery_capacity_kwh / base_power_kw;
            profile += &format!(
                "→ Burst {:.2} minutes peak @ {:.1} MW (supercap boost), then sustained {:.2} hours @ {:.1} MW (battery) — Eternal Surge",
                burst_hours * 60.0,
                requested_power_kw / 1000.0,
                sustained_hours,
                base_power_kw / 1000.0
            );
        } else {
            return Err("Mercy Shield: Requested power exceeds hybrid peak capacity".to_string());
        }

        Ok(profile)
    }
}
