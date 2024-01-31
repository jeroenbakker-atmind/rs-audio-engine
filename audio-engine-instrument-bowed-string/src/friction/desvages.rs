use std::f64::consts::PI;

use crate::bow::{Bow, BOW_FREE_PARAMETER};

use super::{Friction, FrictionData};

#[derive(Default, Debug, Copy, Clone)]
pub struct Desvages {}

/// Desvages friction parameter.
const MUD: f64 = 0.3;

// There seems to be an error in this implementation.
impl Friction for Desvages {
    fn calculate_friction(zeta1: f64, bow: &Bow) -> FrictionData {
        let eta = zeta1 - bow.velocity;
        // d = sqrt(2*a)*exp(-a*eta^2 + 0.5) + 2*muD*atan(eta/0.02)/pi/eta;
        let d = (2.0 * BOW_FREE_PARAMETER).sqrt() * (-BOW_FREE_PARAMETER * eta * eta + 0.5).exp()
            + 2.0 * MUD * (eta / 0.02).atan() / PI / eta;

        // lambda = sqrt(2*a)*exp(-a*eta^2 + 0.5)*(1 - 2*a*eta^2) + 2*muD*50/pi/(2500*eta^2 + 1);
        let lambda = (2.0 * BOW_FREE_PARAMETER).sqrt()
            * (-BOW_FREE_PARAMETER * eta * eta + 0.5).exp()
            * (1.0 - 2.0 * BOW_FREE_PARAMETER * eta * eta)
            + 2.0 * MUD * 50.0 / PI / (2500.0 * eta * eta + 1.0);
        FrictionData { d, lambda }
    }
}
