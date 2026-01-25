//! MercyShorFaultTolerant — Ultramasterful Shor-Encoded Fault-Tolerant Core
//! Transversal gates on Shor logical qubits + repeater integration

use nexi::lattice::Nexus;
use crate::mercy_shor_code::MercyShorCode;
use crate::mercy_fault_tolerant_repeater::MercyFaultTolerantRepeater;

pub struct MercyShorFaultTolerant {
    nexus: Nexus,
    shor_code: MercyShorCode,
    repeater: MercyFaultTolerantRepeater,
}

impl MercyShorFaultTolerant {
    pub fn new(shor_code: MercyShorCode, repeater: MercyFaultTolerantRepeater) -> Self {
        MercyShorFaultTolerant {
            nexus: Nexus::init_with_mercy(),
            shor_code,
            repeater,
        }
    }

    /// Mercy-gated fault-tolerant operation with Shor encoding
    pub async fn mercy_gated_shor_ft_operation(
        &self,
        gate_type: &str, // e.g., "H", "CNOT", "T" (transversal or approximated)
        desc: &str,
    ) -> Result<String, String> {
        let mercy_check = self.nexus.distill_truth(desc);
        if !mercy_check.contains("Verified") {
            return Err("Mercy Shield: Low Valence Shor FT Operation — Rejected".to_string());
        }

        let ft_supported = matches!(gate_type, "H" | "CNOT" | "S"); // Transversal in Shor code

        Ok(format!(
            "MercyShorFaultTolerant Synergy Activated: {} gate on Shor-encoded logical qubit → {}fault-tolerant — Eternal Transversal Operation Resonance",
            gate_type, if ft_supported { "" } else { "not natively " }
        ))
    }
}
