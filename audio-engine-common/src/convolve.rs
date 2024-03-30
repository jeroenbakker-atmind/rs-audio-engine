//! Convolution - common algorithm in audio design to tranport the feeling of one sound to the other.

use std::ops::{AddAssign, Mul};

pub trait Convolution<F> {
    fn convolve(&self, other: &Self) -> Vec<F>;
}

impl<F> Convolution<F> for [F]
where
    F: Copy + Default + AddAssign<F> + Mul<F, Output = F>,
{
    fn convolve(&self, kernel: &Self) -> Vec<F> {
        // Ensure that the kernel is the smaller one.
        if self.len() < kernel.len() {
            return kernel.convolve(self);
        }

        let mut result = Vec::with_capacity(self.len() + kernel.len() - 1);
        let kernel_offset = kernel.len() / 2;
        for index_a in -(kernel_offset as isize) - 1..(self.len() + kernel_offset - 1) as isize {
            let mut res = F::default();
            for index_b in 0..kernel.len() as isize {
                if (0..self.len() as isize).contains(&(index_a + index_b)) {
                    let kernel_index = kernel.len() - 1 - index_b as usize;
                    res += self[(index_a + index_b) as usize] * kernel[kernel_index];
                }
            }
            result.push(res);
        }

        result
    }
}

#[cfg(test)]
mod test {
    use super::Convolution;
    #[test]
    fn convolve_f64() {
        let a = vec![1.0, 1.0, 1.0, 1.0];
        let kernel = vec![0.25, 0.5, 0.25];
        let result = a.convolve(&kernel);
        assert_eq!(result, vec![0.25, 0.75, 1.0, 1.0, 0.75, 0.25]);
    }

    #[test]
    fn convolve_f64_no_mirror() {
        let a = vec![1.0, 1.0, 1.0, 1.0];
        let kernel = vec![0.25, 0.5, 0.35];
        let result = a.convolve(&kernel);
        assert_eq!(result, vec![0.25, 0.75, 1.1, 1.1, 0.85, 0.35]);
    }

    #[test]
    fn convolve_f32() {
        let a = vec![1.0, 1.0, 1.0, 1.0];
        let kernel = vec![0.25_f32, 0.5, 0.25];
        let result = a.convolve(&kernel);
        assert_eq!(result, vec![0.25, 0.75, 1.0, 1.0, 0.75, 0.25]);
    }

    #[test]
    fn convolve_f64_neg() {
        let a = vec![1.0, 1.0, 1.0, 1.0];
        let kernel = vec![0.25, -0.5, 0.25];
        let result = a.convolve(&kernel);
        assert_eq!(result, vec![0.25, -0.25, 0.0, 0.0, -0.25, 0.25]);
    }
}
