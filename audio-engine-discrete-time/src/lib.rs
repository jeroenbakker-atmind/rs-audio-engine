//! Discrete time math library
//!
//! Simulates matlabs tf, tfdata and filter functions.
//! We might want to skip tddata and run the filter directly
//! on the tf struct.

pub mod component;
pub mod components;
pub mod transfer_function;
pub mod filter;

#[cfg(test)]
pub mod transfer_function_test;
