use std::f64::consts::LOG10_E;

pub const LSR_TO_ELO_RATIO: f64 = 400.0 * LOG10_E;
pub const ELO_BASE_SCORE: f64 = 1500.0;

/// Gets the odds of a player winning another, given both LSR ratings.
pub fn odds(winner: f64, loser: f64) -> f64 {
    let gap = loser - winner;
    1.0 / (1.0 + gap.exp())
}

/// Converts an LSR rating into Elo.
pub fn convert_to_elo(lsr: f64) -> f64 {
    lsr * LSR_TO_ELO_RATIO + ELO_BASE_SCORE
}
