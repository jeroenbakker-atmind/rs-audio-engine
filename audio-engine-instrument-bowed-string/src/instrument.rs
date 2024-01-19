use audio_engine_common::digital_sound::{parameters::NoteParameters, sound::Sound};

use crate::{instrument_state::BowedStringInstrumentState, string::String, processor::StringProcessor};

#[derive(Debug, Default, Copy, Clone)]
pub struct BowedStringInstrument {
    pub string: String,
}

impl Sound for BowedStringInstrument {
    type SoundState = BowedStringInstrumentState;
    type Parameters = NoteParameters;

    fn init_sound_state(&self) -> Self::SoundState {
        BowedStringInstrumentState::default()
    }

    fn sample(&self, parameters: &Self::Parameters, state: &mut BowedStringInstrumentState) -> f32 {
        if state.string_processor.is_none() {
            state.string_processor = Some(StringProcessor::new(parameters.sample_rate, &self.string));
        }

        if let Some(processor) = state.string_processor.as_mut() {
            processor.bow.velocity = 0.1;
            processor.bow.pressure = 10.0 * parameters.gain;
            if parameters.note_off.is_some() {
                processor.bow.velocity = 0.0;
                processor.bow.pressure = 0.0;
            }
            processor.compute_state();
            processor.read_output()
        } else {
            0.0
        }
    }
}
