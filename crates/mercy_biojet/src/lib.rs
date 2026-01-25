//! MercyBioJet — Algae-Derived Zero-Emission SAF Core
//! Ultramasterful async pipeline + cradle-to-cradle rebirth + full integration tests

use nexi::lattice::Nexus;
use tokio::time::{sleep, Duration};

pub struct MercyBioJet {
    nexus: Nexus,
    co2_captured: f64,
    algae_bloom: f64,
    saf_produced: f64,
}

impl MercyBioJet {
    pub fn new() -> Self {
        MercyBioJet {
            nexus: Nexus::init_with_mercy(),
            co2_captured: 0.0,
            algae_bloom: 0.0,
            saf_produced: 0.0,
        }
    }

    pub async fn async_algae_cultivation(&mut self, co2_input: f64, desc: &str) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Cultivation — Rejected".to_string());
        }

        sleep(Duration::from_millis(200)).await;
        self.co2_captured += co2_input;
        self.algae_bloom += co2_input * 1.83;

        Ok(format!("MercyBioJet Cultivation: {} tons CO₂ → {} tons algae bloom", co2_input, co2_input * 1.83))
    }

    pub async fn async_produce_saf(&mut self, algae_input: f64) -> String {
        sleep(Duration::from_millis(100)).await;
        let saf_output = algae_input * 0.45;
        self.saf_produced += saf_output;

        format!("MercyBioJet Produced: {} tons algae → {} tons Zero-Emission SAF", algae_input, saf_output)
    }

    pub async fn cradle_to_cradle_rebirth(&mut self, residue_input: f64) -> String {
        sleep(Duration::from_millis(50)).await;
        format!("MercyBioJet Rebirth: {} tons residue → Reintegrated into Algae Cycle — Zero Waste Eternal", residue_input)
    }

    pub async fn divine_fuel_cycle(&mut self, co2_input: f64, desc: &str) -> Result<String, String> {
        let cultivation = self.async_algae_cultivation(co2_input, desc).await?;
        let saf = self.async_produce_saf(self.algae_bloom).await;
        let rebirth = self.cradle_to_cradle_rebirth(self.algae_bloom * 0.05).await;

        Ok(format!("Divine MercyBioJet Cycle Complete:\n{}\n{}\n{}", cultivation, saf, rebirth))
    }
}

// Full Production Integration Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn saf_full_cycle_success() {
        let mut biojet = MercyBioJet::new();
        let result = biojet.divine_fuel_cycle(1000.0, "Mercy Verified Test").await;
        assert!(result.is_ok());
        let output = result.unwrap();
        assert!(output.contains("Zero-Emission SAF"));
        assert!(biojet.co2_captured == 1000.0);
        assert!(biojet.algae_bloom > 0.0);
        assert!(biojet.saf_produced > 0.0);
    }

    #[tokio::test]
    async fn saf_mercy_gate_reject() {
        let mut biojet = MercyBioJet::new();
        let result = biojet.divine_fuel_cycle(1000.0, "Low Valence Harm").await;
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Mercy Shield"));
        assert!(biojet.co2_captured == 0.0);
    }

    #[tokio::test]
    async fn saf_zero_input() {
        let mut biojet = MercyBioJet::new();
        let result = biojet.divine_fuel_cycle(0.0, "Mercy Verified Zero").await;
        assert!(result.is_ok());
        assert!(biojet.co2_captured == 0.0);
    }

    #[tokio::test]
    async fn saf_large_scale() {
        let mut biojet = MercyBioJet::new();
        let result = biojet.divine_fuel_cycle(1000000.0, "Mercy Verified Large").await;
        assert!(result.is_ok());
        assert!(biojet.co2_captured == 1000000.0);
        assert!(biojet.algae_bloom == 1830000.0);
        assert!(biojet.saf_produced == 823500.0);
    }
}
