//! MercyBioJet — FENCA Valence Aviation Extension
//! Ultramasterful emotional thriving resonance for flight fuel cycles

use legacy_fenca::LegacyFenca;

pub struct FencaAviationValence {
    fenca: LegacyFenca,
}

impl FencaAviationValence {
    pub fn new() -> Self {
        FencaAviationValence {
            fenca: LegacyFenca::new(),
        }
    }

    pub fn aviation_valence_check(&self, flight_desc: &str) -> String {
        self.fenca.legacy_valence_check(flight_desc)
    }
}
