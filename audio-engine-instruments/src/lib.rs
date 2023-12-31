use audio_engine_sequencer::instrument::Instrument;
use fm::basic::harmonic::{
    create_fm_basic_harmonic_saw_instrument, create_fm_basic_harmonic_square_instrument,
    create_fm_basic_harmonic_triangle_instrument,
};
use fm::basic::{
    saw_ramp_down::create_fm_basic_saw_ramp_down_instrument,
    saw_ramp_up::create_fm_basic_saw_ramp_up_instrument, sine::create_fm_basic_sine_instrument,
    square::create_fm_basic_square_instrument, triangle::create_fm_basic_triangle_instrument,
};
use fm::wip::create_fm_wip_instrument;

mod fm;
mod sample;

pub enum InstrumentLibrary {
    FmBasicWaveformSine,
    FmBasicWaveformTriangle,
    FmBasicWaveformSawRampUp,
    FmBasicWaveformSawRampDown,
    FmBasicWaveformSquare,

    FmBasicHarmonicTriangle3,
    FmBasicHarmonicTriangle5,
    FmBasicHarmonicTriangle7,
    FmBasicHarmonicTriangle9,
    FmBasicHarmonicTriangle11,
    FmBasicHarmonicTriangle13,
    FmBasicHarmonicTriangle15,
    FmBasicHarmonicSquare3,
    FmBasicHarmonicSquare5,
    FmBasicHarmonicSquare7,
    FmBasicHarmonicSquare9,
    FmBasicHarmonicSquare11,
    FmBasicHarmonicSquare13,
    FmBasicHarmonicSquare15,
    FmBasicHarmonicSaw3,
    FmBasicHarmonicSaw5,
    FmBasicHarmonicSaw7,
    FmBasicHarmonicSaw9,
    FmBasicHarmonicSaw11,
    FmBasicHarmonicSaw13,
    FmBasicHarmonicSaw15,

    FmWIP,
}

impl InstrumentLibrary {
    pub fn create(&self) -> Instrument {
        match self {
            Self::FmBasicWaveformSine => create_fm_basic_sine_instrument(),
            Self::FmBasicWaveformTriangle => create_fm_basic_triangle_instrument(),
            Self::FmBasicWaveformSawRampUp => create_fm_basic_saw_ramp_up_instrument(),
            Self::FmBasicWaveformSawRampDown => create_fm_basic_saw_ramp_down_instrument(),
            Self::FmBasicWaveformSquare => create_fm_basic_square_instrument(),

            Self::FmBasicHarmonicTriangle3 => create_fm_basic_harmonic_triangle_instrument(3),
            Self::FmBasicHarmonicTriangle5 => create_fm_basic_harmonic_triangle_instrument(5),
            Self::FmBasicHarmonicTriangle7 => create_fm_basic_harmonic_triangle_instrument(7),
            Self::FmBasicHarmonicTriangle9 => create_fm_basic_harmonic_triangle_instrument(9),
            Self::FmBasicHarmonicTriangle11 => create_fm_basic_harmonic_triangle_instrument(11),
            Self::FmBasicHarmonicTriangle13 => create_fm_basic_harmonic_triangle_instrument(13),
            Self::FmBasicHarmonicTriangle15 => create_fm_basic_harmonic_triangle_instrument(15),
            Self::FmBasicHarmonicSquare3 => create_fm_basic_harmonic_square_instrument(3),
            Self::FmBasicHarmonicSquare5 => create_fm_basic_harmonic_square_instrument(5),
            Self::FmBasicHarmonicSquare7 => create_fm_basic_harmonic_square_instrument(7),
            Self::FmBasicHarmonicSquare9 => create_fm_basic_harmonic_square_instrument(9),
            Self::FmBasicHarmonicSquare11 => create_fm_basic_harmonic_square_instrument(11),
            Self::FmBasicHarmonicSquare13 => create_fm_basic_harmonic_square_instrument(13),
            Self::FmBasicHarmonicSquare15 => create_fm_basic_harmonic_square_instrument(15),
            Self::FmBasicHarmonicSaw3 => create_fm_basic_harmonic_saw_instrument(3),
            Self::FmBasicHarmonicSaw5 => create_fm_basic_harmonic_saw_instrument(5),
            Self::FmBasicHarmonicSaw7 => create_fm_basic_harmonic_saw_instrument(7),
            Self::FmBasicHarmonicSaw9 => create_fm_basic_harmonic_saw_instrument(9),
            Self::FmBasicHarmonicSaw11 => create_fm_basic_harmonic_saw_instrument(11),
            Self::FmBasicHarmonicSaw13 => create_fm_basic_harmonic_saw_instrument(13),
            Self::FmBasicHarmonicSaw15 => create_fm_basic_harmonic_saw_instrument(15),

            Self::FmWIP => create_fm_wip_instrument(),
        }
    }
}
