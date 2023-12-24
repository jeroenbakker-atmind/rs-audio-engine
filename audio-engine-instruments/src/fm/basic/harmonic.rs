use audio_engine_common::waveform::Waveform;
use audio_engine_sequencer::instrument::Instrument;

use super::create_fm_waveform_instrument;

pub fn create_fm_basic_harmonic_square_instrument(harmonic: u8) -> Instrument {
    create_fm_waveform_instrument(Waveform::Morph(0.0, 1.0, harmonic))
}

pub fn create_fm_basic_harmonic_triangle_instrument(harmonic: u8) -> Instrument {
    create_fm_waveform_instrument(Waveform::Morph(1.0, 0.0, harmonic))
}

pub fn create_fm_basic_harmonic_saw_instrument(harmonic: u8) -> Instrument {
    create_fm_waveform_instrument(Waveform::Morph(1.0, 1.0, harmonic))
}


