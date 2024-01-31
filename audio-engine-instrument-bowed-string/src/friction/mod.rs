use crate::bow::Bow;

pub mod bilboa;
pub mod desvages;

#[derive(Debug)]
pub struct FrictionData {
    pub d: f64,
    pub lambda: f64,
}

/// Trait to make the used friction model plugable in the string processor.
pub trait Friction {
    fn calculate_friction(zeta1: f64, bow: &Bow) -> FrictionData;
}
