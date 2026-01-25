//! MercyCarbonCapture — Direct Air + Exhaust CO₂ Capture Core
//! Ultramasterful cradle-to-cradle resonance for eternal MercyBioJet rebirth

use nexi::lattice::Nexus;
use mercy_biojet::MercyBioJet;
use tokio::time::{sleep, Duration};

pub struct MercyCarbonCapture {
    nexus: Nexus,
    biojet: MercyBioJet,
}

impl MercyCarbonCapture {
    pub fn new() -> Self {
        MercyCarbonCapture {
            nexus: Nexus::init_with_mercy(),
            biojet: MercyBioJet::new(),
        }
    }

    /// Mercy-gated direct air capture (DAC) + algae feed
    pub async fn mercy_gated_dac_capture(&mut self, co2_input: f64, desc: &str) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence DAC Input — Capture Rejected".to_string());
        }

        sleep(Duration::from_millis(150)).await; // DAC latency
        self.biojet.async_algae_cultivation(co2_input, desc).await
    }

    /// Mercy-gated industrial exhaust capture + algae feed
    pub async fn mercy_gated_exhaust_capture(&mut self, co2_input: f64, desc: &str) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Exhaust Input — Capture Rejected".to_string());
        }

        sleep(Duration::from_millis(100)).await; // Exhaust capture latency
        self.biojet.async_algae_cultivation(co2_input, desc).await
    }

    /// Full async divine capture cycle
    pub async fn divine_capture_cycle(&mut self, co2_input: f64, source: &str, desc: &str) -> Result<String, String> {
        match source {
            "dac" => self.mercy_gated_dac_capture(co2_input, desc).await,
            "exhaust" => self.mercy_gated_exhaust_capture(co2_input, desc).await,
            _ => Err("Invalid Capture Source".to_string()),
        }
    }
}
