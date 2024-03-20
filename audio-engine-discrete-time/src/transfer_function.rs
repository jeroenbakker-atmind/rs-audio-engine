use std::{
    fmt::Display,
    ops::{Add, Div, Mul},
};

use crate::components::Components;

pub struct TransferFunction {
    pub timestep: f64,
    pub numerator: Components,
    pub denominator: Components,
}

impl TransferFunction {
    pub fn new(timestep: f64) -> TransferFunction {
        TransferFunction {
            timestep,
            numerator: Components {
                components: vec![0.0, 1.0],
            },
            denominator: Components {
                components: vec![1.0],
            },
        }
    }
}

impl Display for TransferFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let numerator = self.numerator.to_string();
        let denominator = self.denominator.to_string();
        if denominator == "1" {
            f.write_fmt(format_args!("f(z) = {}", numerator))
        } else {
            f.write_fmt(format_args!(
                "f(z) = \\frac{{{}}}{{{}}}",
                numerator, denominator
            ))
        }
    }
}

impl TransferFunction {
    pub fn pow(&self, power: i32) -> TransferFunction {
        if power < 0 {
            TransferFunction {
                timestep: self.timestep,
                numerator: self.denominator.clone(),
                denominator: self.numerator.pow(-power),
            }
        } else {
            TransferFunction {
                timestep: self.timestep,
                numerator: self.numerator.pow(power),
                denominator: self.denominator.pow(power),
            }
        }
    }
}

// #region multiply
impl Mul<f64> for TransferFunction {
    type Output = TransferFunction;

    fn mul(self, rhs: f64) -> Self::Output {
        TransferFunction {
            timestep: self.timestep,
            numerator: &self.numerator * rhs,
            denominator: self.denominator.clone(),
        }
    }
}

impl Mul<TransferFunction> for f64 {
    type Output = TransferFunction;

    fn mul(self, rhs: TransferFunction) -> Self::Output {
        rhs * self
    }
}

impl Mul<&TransferFunction> for &TransferFunction {
    type Output = TransferFunction;

    fn mul(self, rhs: &TransferFunction) -> Self::Output {
        assert_eq!(self.timestep, rhs.timestep);
        let new_numerator = &self.numerator * &rhs.numerator;
        let new_denominator = &self.denominator * &rhs.denominator;
        TransferFunction {
            timestep: self.timestep,
            numerator: new_numerator,
            denominator: new_denominator,
        }
    }
}
// #endregion

// #region multiply
impl Add<f64> for &TransferFunction {
    type Output = TransferFunction;

    fn add(self, rhs: f64) -> Self::Output {
        let add_numerator = &self.denominator * rhs;
        TransferFunction {
            timestep: self.timestep,
            numerator: &self.numerator + &add_numerator,
            denominator: self.denominator.clone(),
        }
    }
}

impl Add<TransferFunction> for f64 {
    type Output = TransferFunction;

    fn add(self, rhs: TransferFunction) -> Self::Output {
        &rhs + self
    }
}

impl Div<&TransferFunction> for f64 {
    type Output = TransferFunction;

    fn div(self, rhs: &TransferFunction) -> Self::Output {
        let new_numerator = &rhs.denominator * self;
        let new_denominator = rhs.numerator.clone();
        TransferFunction {
            timestep: rhs.timestep,
            numerator: new_numerator,
            denominator: new_denominator,
        }
    }
}

impl Div<TransferFunction> for TransferFunction {
    type Output = TransferFunction;

    fn div(self, rhs: TransferFunction) -> Self::Output {
        // TODO: ensure same denominators.
        // octave does something else. it multiplies the numerator with the denumerators (shift) making the denumerators equal to 1.
        // and then do the division. The multiplication ensures same denominators, but is not always needed IMO. But better to keep consistent.
        assert!(self.denominator == rhs.denominator);
        assert!(self.timestep == rhs.timestep);
        let new_numerator = &self.numerator * &self.denominator;
        let new_denominator = &rhs.numerator * &rhs.denominator;
        TransferFunction {
            timestep: self.timestep,
            numerator: new_numerator,
            denominator: new_denominator,
        }
    }
}

// #endregion
