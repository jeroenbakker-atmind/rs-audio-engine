use std::str::FromStr;

use crate::{tone::Tone, ChromaticScale};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ChromaticTone {
    C,
    CSharp,
    D,
    DSharp,
    E,
    F,
    FSharp,
    G,
    GSharp,
    A,
    ASharp,
    B,
}

impl Tone for ChromaticTone {
    type ScaleType = ChromaticScale;

    fn scale() -> Self::ScaleType {
        ChromaticScale::default()
    }
}

impl From<u8> for ChromaticTone {
    fn from(value: u8) -> Self {
        match value {
            0 => ChromaticTone::C,
            1 => ChromaticTone::CSharp,
            2 => ChromaticTone::D,
            3 => ChromaticTone::DSharp,
            4 => ChromaticTone::E,
            5 => ChromaticTone::F,
            6 => ChromaticTone::FSharp,
            7 => ChromaticTone::G,
            8 => ChromaticTone::GSharp,
            9 => ChromaticTone::A,
            10 => ChromaticTone::ASharp,
            11 => ChromaticTone::B,
            _ => unreachable!(),
        }
    }
}

impl From<ChromaticTone> for u8 {
    fn from(value: ChromaticTone) -> Self {
        match value {
            ChromaticTone::C => 0,
            ChromaticTone::CSharp => 1,
            ChromaticTone::D => 2,
            ChromaticTone::DSharp => 3,
            ChromaticTone::E => 4,
            ChromaticTone::F => 5,
            ChromaticTone::FSharp => 6,
            ChromaticTone::G => 7,
            ChromaticTone::GSharp => 8,
            ChromaticTone::A => 9,
            ChromaticTone::ASharp => 10,
            ChromaticTone::B => 11,
        }
    }
}

impl ChromaticTone {
    // double NOTE_UP_SCALAR = pow(2.0,1.0/12.0);
    // f(t) = f(t-1) * note_up_scalar;
    pub const fn pitch_octave4(&self) -> f32 {
        match self {
            ChromaticTone::C => 261.63,
            ChromaticTone::CSharp => 277.18,
            ChromaticTone::D => 293.66,
            ChromaticTone::DSharp => 311.13,
            ChromaticTone::E => 329.63,
            ChromaticTone::F => 349.23,
            ChromaticTone::FSharp => 369.99,
            ChromaticTone::G => 392.00,
            ChromaticTone::GSharp => 415.30,
            ChromaticTone::A => 440.00,
            ChromaticTone::ASharp => 466.16,
            ChromaticTone::B => 493.88,
        }
    }
}

impl FromStr for ChromaticTone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tone = match s {
            "C" => ChromaticTone::C,
            "C#" => ChromaticTone::CSharp,
            "D" => ChromaticTone::D,
            "D#" => ChromaticTone::DSharp,
            "E" => ChromaticTone::E,
            "F" => ChromaticTone::F,
            "F#" => ChromaticTone::FSharp,
            "G" => ChromaticTone::G,
            "G#" => ChromaticTone::GSharp,
            "A" => ChromaticTone::A,
            "A#" => ChromaticTone::ASharp,
            "B" => ChromaticTone::B,
            _ => {
                return Err(());
            }
        };
        Ok(tone)
    }
}
