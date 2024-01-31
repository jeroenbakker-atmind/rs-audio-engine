use crate::bow::{Bow, BOW_FREE_PARAMETER};

use super::{Friction, FrictionData};

#[derive(Default, Debug, Copy, Clone)]
pub struct Bilbao {}

impl Friction for Bilbao {
    fn calculate_friction(zeta1: f64, bow: &Bow) -> FrictionData {
        let eta = zeta1 - bow.velocity;
        let d = (2.0 * BOW_FREE_PARAMETER).sqrt() * (-BOW_FREE_PARAMETER * eta * eta + 0.5).exp();
        let lambda = d * (1.0 - 2.0 * BOW_FREE_PARAMETER * eta.powi(2));
        FrictionData { d, lambda }
    }
}
