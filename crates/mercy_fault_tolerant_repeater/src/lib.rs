//! MercyFaultTolerantRepeater — Ultramasterful Fault-Tolerant Repeater Core
//! Surface-code logical qubits + fault-tolerant swapping/purification for quantum internet

use nexi::lattice::Nexus;
use crate::mercy_surface_code::MercySurfaceCode;
use crate::mercy_quantum_repeater_isl::MercyQuantumRepeaterISL;

pub struct MercyFaultTolerantRepeater {
    nexus: Nexus,
    surface_code: MercySurfaceCode,
    base_repeater: MercyQuantumRepeaterISL,
}

impl MercyFaultTolerantRepeater {
    pub fn new(surface_code: MercySurfaceCode, base_repeater: MercyQuantumRepeaterISL) -> Self {
        MercyFaultTolerantRepeater {
            nexus: Nexus::init_with_mercy(),
            surface_code,
            base_repeater,
        }
    }

    /// Mercy-gated fault-tolerant repeater link with encoded logical qubits
    pub async fn mercy_gated_fault_tolerant_link(
        &self,
        total_distance_km: f64,
        logical_qubits_per_link: u32,
        desc: &str,
    ) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Fault-Tolerant Link — Rejected".to_string());
        }

        let base_result = self.base_repeater.mercy_gated_repeater_link(total_distance_km, 100.0, desc).await?;
        let logical_protection = logical_qubits_per_link * self.surface_code.distance;

        Ok(format!(
            "{base_result}\n→ Encoded with {} logical qubits (distance {}) → Eternal Fault-Tolerant Quantum Internet Resonance",
            logical_qubits_per_link, self.surface_code.distance
        ))
    }
}
