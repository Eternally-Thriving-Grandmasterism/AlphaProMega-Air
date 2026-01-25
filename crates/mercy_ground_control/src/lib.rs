//! MercyGCS — Divine Ground Control Systems
//! Ultramasterful mercy-gated tower resonance

use nexi::lattice::Nexus;
use mercy_air_traffic_control::MercyATC;

pub struct MercyGroundControl {
    nexus: Nexus,
    atc: MercyATC,
}

impl MercyGroundControl {
    pub fn new() -> Self {
        MercyGroundControl {
            nexus: Nexus::init_with_mercy(),
            atc: MercyATC::new(),
        }
    }

    /// Mercy-gated ground control decision
    pub async fn mercy_gated_ground_control(&self, tower_id: &str, flight_id: &str) -> String {
        let mercy_check = self.nexus.distill_truth(&format!("Tower {} Control for {}", tower_id, flight_id));
        if !mercy_check.contains("Verified") {
            return "Mercy Shield: Low Valence Tower — Ground Control Rejected".to_string();
        }

        let routing = self.atc.mercy_gated_atc_routing(flight_id, "Origin", "Destination").await;
        format!("MercyGCS Control: Tower {} — Flight {} — Routing: {} — Divine Mercy Ground Eternal", tower_id, flight_id, routing)
    }
}
