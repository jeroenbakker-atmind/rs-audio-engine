use std::ops::{AddAssign, Mul, SubAssign};

use crate::transfer_function::TransferFunction;

#[derive(Debug, Clone)]
pub struct Filter<F> {
    pub x: Vec<F>,
    pub y: Vec<F>,
    pub a: Vec<F>,
    pub b: Vec<F>,
    pub n: usize,
}

impl<F> Filter<F>
where
    F: Copy + Default + Mul<F, Output = F> + AddAssign<F> + SubAssign<F>,
{
    pub fn filter(&mut self, value_in: F) -> F {
        self.x.pop().unwrap();
        self.x.insert(0, value_in);
        self.y.pop().unwrap();
        self.y.insert(0, F::default());
        let mut result = self.b[0] * value_in;
        for index in 1..=self.n as usize {
            result += self.b[index] * self.x[index];
            result -= self.a[index] * self.y[index];
        }
        self.y[0] = result;
        result
    }
}

// `see https://github.com/gnu-octave/octave/blob/d373693c1b797ded2ce9068c800563d0ff380406/libinterp/corefcn/filter.cc#L48` for backing function.
impl From<TransferFunction> for Filter<f64> {
    fn from(transfer_function: TransferFunction) -> Self {
        let n = transfer_function
            .denominator
            .len()
            .max(transfer_function.numerator.len());

        let mut a = transfer_function.denominator.components.clone();
        let mut b = transfer_function.numerator.components.clone();
        a.reverse();
        b.reverse();
        a.resize(n + 1, 0.0);
        b.resize(n + 1, 0.0);
        let x = vec![0.0; n + 1];
        let y = vec![0.0; n + 1];
        Filter { a, b, x, y, n }
    }
}
