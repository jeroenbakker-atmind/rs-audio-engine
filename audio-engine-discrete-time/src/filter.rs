use crate::transfer_function::TransferFunction;

#[derive(Debug)]
pub struct Filter {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub a: Vec<f64>,
    pub b: Vec<f64>,
    pub n: usize,
}

impl Filter {
    pub fn filter(&mut self, value_in: f64) -> f64 {
        self.x.pop().unwrap();
        self.x.insert(0, value_in);
        self.y.pop().unwrap();
        self.y.insert(0, 0.0);
        let mut result = self.b[0] * value_in;
        for index in 1..=self.n as usize {
            result += self.b[index] * self.x[index];
            result -= self.a[index] * self.y[index];
        }
        self.y[0] = result;
        result
    }
}

impl From<&TransferFunction> for Filter {
    fn from(transfer_function: &TransferFunction) -> Self {
        let n = transfer_function
            .denominator
            .len()
            .max(transfer_function.numerator.len());

        let mut a = transfer_function.numerator.components.clone();
        let mut b = transfer_function.denominator.components.clone();
        a.resize(n + 1, 0.0);
        b.resize(n + 1, 0.0);
        a.reverse();
        b.reverse();
        let x = vec![0.0; n + 1];
        let y = vec![0.0; n + 1];
        Filter { a, b, x, y, n }
    }
}
