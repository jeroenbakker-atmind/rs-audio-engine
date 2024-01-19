use std::f32::consts::PI;

use audio_engine_notes::Pitch;

#[derive(Debug, Default, Copy, Clone)]
pub struct String {
    /// Radius of the string in meters.
    pub radius: f32,
    /// Density of the string in kg/m.
    pub density: f32,
    /// Tension of the string in newtons.
    pub tension: f32,
    /// Young modulus of the string in Pa.
    pub young_mod: f32,
    /// Length of the string in meters.
    pub length: f32,
}

impl String {
    pub fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }

    pub fn lin_density(&self) -> f32 {
        self.density * self.area()
    }

    pub fn inertia(&self) -> f32 {
        PI * self.radius * self.radius * self.radius * self.radius / 4.0
    }

    pub fn c(&self) -> f32 {
        self.tension / self.lin_density()
    }
}

/// Calculate the hand position from the base pitch of the string and the pitch of the note.
///
/// The multiplier is the location along the string where the string should be shortened.
/// 1.0 means no shortening,
/// 0.0 means no string left to shorten.
pub fn calc_hand_position_multiplier(base_pitch: Pitch, note_pitch: Pitch) -> f32 {
    (base_pitch.frequency / note_pitch.frequency) as f32
}

#[test]
fn hand_position_pitch() {
    let base_pitch = Pitch::from(440.0);
    let semitone1_pitch = Pitch::from(880.0);
    let semitone2_pitch = Pitch::from(1760.0);

    assert_eq!(calc_hand_position_multiplier(base_pitch, base_pitch), 1.0);
    assert_eq!(
        calc_hand_position_multiplier(base_pitch, semitone1_pitch),
        0.5
    );
    assert_eq!(
        calc_hand_position_multiplier(base_pitch, semitone2_pitch),
        0.25
    );
}
