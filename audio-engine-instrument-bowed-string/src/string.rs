use std::f64::consts::PI;

use audio_engine_notes::Pitch;

pub static CELLO_STRING_A3: String = String {
    radius: 3.75e-04,
    density: 3.7575e3,
    tension: 153.0,
    young_mod: 25e9,
    length: 0.69,
    excit_position: InOutPosition::Fixed(0.833),
    output_position_left: InOutPosition::Factor(0.33),
    output_position_right: InOutPosition::Factor(0.77),
};
pub static CELLO_STRING_D3: String = String {
    radius: 4.4e-04,
    density: 4.1104e3,
    tension: 102.6,
    young_mod: 25e9,
    length: 0.69,
    excit_position: InOutPosition::Fixed(0.833),
    output_position_left: InOutPosition::Factor(0.33),
    output_position_right: InOutPosition::Factor(0.77),
};
pub static CELLO_STRING_G2: String = String {
    radius: 6.05e-04,
    density: 5.3570e3,
    tension: 112.67,
    young_mod: 8.6e9,
    length: 0.69,
    excit_position: InOutPosition::Factor(0.733),
    output_position_left: InOutPosition::Factor(0.53),
    output_position_right: InOutPosition::Factor(0.77),
};
pub static CELLO_STRING_C2: String = String {
    radius: 7.2e-4,
    density: 1.3017e4,
    tension: 172.74,
    young_mod: 22.4e9,
    length: 0.69,
    excit_position: InOutPosition::Factor(0.733),
    output_position_left: InOutPosition::Factor(0.33),
    output_position_right: InOutPosition::Factor(0.57),
};

pub static VIOLIN_STRING_E5: String = String {
    radius: 1.65e-04,
    density: 4.7936e3,
    tension: 73.0,
    young_mod: 62.5e9,
    length: 0.32,
    excit_position: InOutPosition::Fixed(0.833),
    output_position_left: InOutPosition::Factor(0.33),
    output_position_right: InOutPosition::Factor(0.77),
};

pub static VIOLIN_STRING_A4: String = String {
    radius: 3e-4,
    density: 2.5465e+3,
    tension: 57.1,
    young_mod: 19.5e9,
    length: 0.32,
    excit_position: InOutPosition::Fixed(0.833),
    output_position_left: InOutPosition::Factor(0.33),
    output_position_right: InOutPosition::Factor(0.77),
};
pub static VIOLIN_STRING_D4: String = String {
    radius: 4.4e-4,
    density: 2.6471e3,
    tension: 56.88,
    young_mod: 4.56e9,
    length: 0.32,
    excit_position: InOutPosition::Factor(0.733),
    output_position_left: InOutPosition::Factor(0.53),
    output_position_right: InOutPosition::Factor(0.77),
};
pub static VIOLIN_STRING_G3: String = String {
    radius: 4.25e-4,
    density: 4.9167e3,
    tension: 43.9,
    young_mod: 4.79e9,
    length: 0.32,
    excit_position: InOutPosition::Factor(0.733),
    output_position_left: InOutPosition::Factor(0.53),
    output_position_right: InOutPosition::Factor(0.77),
};

#[derive(Debug, Copy, Clone)]
pub enum InOutPosition {
    Fixed(f64),
    Factor(f64),
}

impl InOutPosition {
    pub fn get_value(&self, length: f64) -> f64 {
        match self {
            InOutPosition::Factor(factor) => length * factor,
            InOutPosition::Fixed(value) => *value,
        }
    }
}

impl Default for InOutPosition {
    fn default() -> Self {
        InOutPosition::Fixed(0.833)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct String {
    /// Radius of the string in meters.
    pub radius: f64,
    /// Density of the string in kg/m.
    pub density: f64,
    /// Tension of the string in newtons.
    pub tension: f64,
    /// Young modulus of the string in Pa.
    pub young_mod: f64,
    /// Length of the string in meters.
    pub length: f64,

    pub excit_position: InOutPosition,
    pub output_position_left: InOutPosition,
    pub output_position_right: InOutPosition,
}

impl String {
    pub fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    pub fn lin_density(&self) -> f64 {
        self.density * self.area()
    }

    pub fn inertia(&self) -> f64 {
        PI * self.radius * self.radius * self.radius * self.radius / 4.0
    }
}

/// Calculate the hand position from the base pitch of the string and the pitch of the note.
///
/// The multiplier is the location along the string where the string should be shortened.
/// 1.0 means no shortening,
/// 0.0 means no string left to shorten.
pub fn calc_hand_position_multiplier(base_pitch: Pitch, note_pitch: Pitch) -> f64 {
    (base_pitch.frequency / note_pitch.frequency) as f64
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
