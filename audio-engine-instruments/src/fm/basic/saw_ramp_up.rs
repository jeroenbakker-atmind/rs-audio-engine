use audio_engine_common::waveform::Waveform;
use audio_engine_sequencer::instrument::Instrument;

use super::create_fm_waveform_instrument;

pub fn create_fm_basic_saw_ramp_up_instrument() -> Instrument {
    create_fm_waveform_instrument(Waveform::Saw(false))
}
