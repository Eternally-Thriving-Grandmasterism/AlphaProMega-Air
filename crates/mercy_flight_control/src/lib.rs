pub struct MercyFlightControl {
    nexus: Nexus,
}

impl MercyFlightControl {
    pub fn new() -> Self {
        MercyFlightControl {
            nexus: Nexus::init_with_mercy(),
        }
    }

    pub async fn mercy_autopilot(&self, phase: &str) -> String {
        let check = self.nexus.distill_truth(phase);
        if !check.contains("Verified") {
            return "Mercy Shield: Low Valence Flight Path — Abort".to_string();
        }
        format!("MercyFlightControl Active: Phase — Valence-Gated Maneuver", phase)
    }
}
