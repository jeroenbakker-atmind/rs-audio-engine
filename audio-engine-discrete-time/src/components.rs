use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut, Mul},
};

use crate::component::Component;

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Components {
    pub components: Vec<f64>,
}

impl Components {
    pub fn pow(&self, exponent: i32) -> Components {
        let mut result = self.clone();
        for _ in 1..exponent {
            result = self * &result;
        }
        result
    }
}

impl Index<usize> for Components {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.components[index]
    }
}
impl IndexMut<usize> for Components {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if self.components.len() < index + 1 {
            self.components.resize(index + 1, 0.0);
        }
        &mut self.components[index]
    }
}

impl Display for Components {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;

        for (power, factor) in self.components.iter().enumerate() {
            let component = Component::new(power as i32, *factor);

            let str = format!("{}", component);
            if str.is_empty() {
                continue;
            }
            if !first {
                f.write_str(" ")?;
            }
            if !first && component.factor.is_sign_positive() {
                f.write_str("+")?;
            }
            f.write_str(&str)?;
            first = false;
        }
        Ok(())
    }
}

impl Mul<f64> for &Components {
    type Output = Components;

    fn mul(self, rhs: f64) -> Self::Output {
        Components {
            components: self.components.iter().map(|factor| *factor * rhs).collect(),
        }
    }
}

impl Mul<&Components> for &Components {
    type Output = Components;

    fn mul(self, rhs: &Components) -> Self::Output {
        let mut result = Components::default();
        for (index_a, component_a) in self.components.iter().enumerate() {
            for (index_b, component_b) in rhs.components.iter().enumerate() {
                result[index_a + index_b] += component_a * component_b;
            }
        }
        result
    }
}

impl<'a, C: Into<&'a Components> + Sized> Add<C> for &Components {
    type Output = Components;

    fn add(self, rhs: C) -> Self::Output {
        let rhs = rhs.into();
        let mut result = Components {
            components: vec![0.0; self.components.len().max(rhs.components.len())],
        };

        result
            .components
            .iter_mut()
            .zip(&self.components)
            .for_each(|(a, b)| {
                *a += b;
            });

        result
            .components
            .iter_mut()
            .zip(&rhs.components)
            .for_each(|(a, b)| {
                *a += b;
            });

        result
    }
}
