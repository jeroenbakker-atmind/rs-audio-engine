//! Discrete time math library
//!
//! Simulates matlabs tf, tfdata and filter functions.
//! The implementation is limited in scope. Only operations needed by other crates have been
//! implemented.
//! 
//! ## TODO
//! We might want to skip tddata and run the filter directly
//! on the tf struct.

pub mod component;
pub mod components;
pub mod filter;
pub mod transfer_function;

#[cfg(test)]
pub mod transfer_function_test;
