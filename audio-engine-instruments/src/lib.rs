use audio_engine_sequencer::instrument::Instrument;
use fm::basic::{
    saw_ramp_down::create_fm_basic_saw_ramp_down_instrument,
    saw_ramp_up::create_fm_basic_saw_ramp_up_instrument, sine::create_fm_basic_sine_instrument,
    square::create_fm_basic_square_instrument, triangle::create_fm_basic_triangle_instrument,
};

mod fm;

pub enum InstrumentLibrary {
    FmBasicWaveformSine,
    FmBasicWaveformTriangle,
    FmBasicWaveformSawRampUp,
    FmBasicWaveformSawRampDown,
    FmBasicWaveformSquare,
}

impl InstrumentLibrary {
    pub fn create(&self) -> Instrument {
        match self {
            Self::FmBasicWaveformSine => create_fm_basic_sine_instrument(),
            Self::FmBasicWaveformTriangle => create_fm_basic_triangle_instrument(),
            Self::FmBasicWaveformSawRampUp => create_fm_basic_saw_ramp_up_instrument(),
            Self::FmBasicWaveformSawRampDown => create_fm_basic_saw_ramp_down_instrument(),
            Self::FmBasicWaveformSquare => create_fm_basic_square_instrument(),
        }
    }
}
