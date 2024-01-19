use audio_engine_common::digital_sound::{parameters::NoteParameters, sound::Sound};

use crate::{instrument_state::BowedStringInstrumentState, string::String, processor::StringProcessor, bow::Bow};

#[derive(Debug, Default, Clone)]
pub struct BowedStringInstrument {
    pub strings: Vec<String>,
}

impl BowedStringInstrument {
    pub fn add_string(&mut self, string: String) {
        self.strings.push(string);
    }
}

impl Sound for BowedStringInstrument {
    type SoundState = BowedStringInstrumentState;
    type Parameters = NoteParameters;

    fn init_sound_state(&self) -> Self::SoundState {
        BowedStringInstrumentState::default()
    }

    fn sample(&self, parameters: &Self::Parameters, state: &mut BowedStringInstrumentState) -> f32 {
        if state.string_processors.is_empty() {
            for string in &self.strings {
                state.string_processors.push(StringProcessor::new(parameters.sample_rate, string));

            }
        }

        // Remove pressure from all strings
        state.string_processors.iter_mut().for_each(|processor|processor.bow = Bow{velocity: 0.0, pressure: 0.0});
        
        // TODO: Select a string, closest to the last played note.
        // TODO: apply pressure and hand position to the string.
        let processor = &mut state.string_processors[2];
        processor.bow.velocity = 0.1;
        processor.bow.pressure = 10.0 * parameters.gain;
        if parameters.note_off.is_some() {
            processor.bow.velocity = 0.0;
            processor.bow.pressure = 0.0;
        }
        
        // Sample the state of all strings 
        state.string_processors.iter_mut().for_each(|processor|processor.compute_state());
        state.string_processors.iter_mut().map(|processor|processor.read_output()).sum::<f32>()
    }
}
