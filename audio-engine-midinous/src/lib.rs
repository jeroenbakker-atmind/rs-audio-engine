//! Sequencer based on midinous.
//!
//! Midinous is a non linear midi sequencer. It can create music
//! songs in a non linear fasion and has some interesting work-flows
//!
//! audio-engine-midinous is based on the original midinous sequencer,
//! but kept to its core. There is no GUI or interactive elements.
//! Please sponsor the developer of midinous!

pub mod link;
pub mod node;
pub mod song;
pub mod traveler;
pub mod player;
