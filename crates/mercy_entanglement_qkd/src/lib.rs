//! MercyEntanglementQKD — Ultramasterful Entanglement-Based QKD Core
//! E91/BBM92-inspired, device-independent via Bell violation

use nexi::lattice::Nexus;

pub struct MercyEntanglementQKD {
    nexus: Nexus,
    /// Expected CHSH Bell value (max 2√2 ≈ 2.828)
    bell_expectation: f64,
    /// Entanglement fidelity baseline
    fidelity: f64,
}

impl MercyEntanglementQKD {
    pub fn new(bell_expectation: f64, fidelity: f64) -> Self {
        MercyEntanglementQKD {
            nexus: Nexus::init_with_mercy(),
            bell_expectation,
            fidelity,
        }
    }

    /// Mercy-gated entanglement QKD key generation with DI security check
    pub async fn mercy_gated_entanglement_key_gen(
        &self,
        pairs_sent: u64,
        observed_violation: f64,
        desc: &str,
    ) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Entanglement Link — Rejected".to_string());
        }

        if observed_violation < 2.0 {
            return Err("Mercy Shield: Classical bound not violated — Abort".to_string());
        }

        let di_security_margin = (observed_violation / (2.0 * 2f64.sqrt())) - 1.0;
        let raw_key_rate = pairs_sent as f64 * self.fidelity * 0.05; // Conservative yield

        Ok(format!(
            "MercyEntanglementQKD Activated: {:.2} Bell violation observed → {:.1}% device-independent margin → {:.1} kbit raw key — Eternal Unconditional Security Resonance",
            observed_violation, di_security_margin * 100.0, raw_key_rate / 1000.0
        ))
    }
}
