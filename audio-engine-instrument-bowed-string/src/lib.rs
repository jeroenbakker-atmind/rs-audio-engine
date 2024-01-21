pub mod bow;
pub mod damping;
pub mod eigen_frequencies;
pub mod hand;
pub mod instrument;
pub mod instrument_state;
pub mod nemus_processor;
pub mod nemus_try_processor;
pub mod processor;
pub mod sherman_morrison_processor;
pub mod string;
pub mod string_and_hand;

#[cfg(test)]
mod test;

// TODO: generate waveform png based on a property.
// What happens when the fretting position changes.
// What happens when the bow velocity changes.
// What happens when the bow pressure changes.
// TODO: Add low pass filter, remove all frequencies above 20Khz
// TODO: When two notes are played in sequence the bow speed might switch direction?