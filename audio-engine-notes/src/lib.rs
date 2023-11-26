//! Music notes is a rust library for using musical notes on a data-level.
//! 
//! ## History
//! 

mod chromatic;
mod note;
mod octave;
mod pitch;
mod scale;
mod tone;

pub use chromatic::*;
pub use note::*;
pub use octave::*;
pub use pitch::*;
pub use scale::*;
pub use tone::*;
