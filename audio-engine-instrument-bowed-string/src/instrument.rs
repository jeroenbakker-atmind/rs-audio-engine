use std::cmp::Ordering;

use audio_engine_common::digital_sound::{parameters::NoteParameters, sound::Sound};
use audio_engine_notes::Pitch;

use crate::{
    bow::Bow,
    instrument_state::BowedStringInstrumentState,
    processor::StringProcessor,
    string::{calc_hand_position_multiplier, String},
};

#[derive(Debug, Default, Clone)]
pub struct BowedStringInstrument {
    pub strings: Vec<String>,
    /// Thae base pitch for each string in strings attribute.
    pub string_pitches: Vec<Pitch>,
}

impl BowedStringInstrument {
    pub fn add_string(&mut self, string: String, string_pitch: Pitch) {
        self.strings.push(string);
        self.string_pitches.push(string_pitch)
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
                state
                    .string_processors
                    .push(StringProcessor::new(parameters.sample_rate, string));
            }
        }

        // Remove pressure from all strings
        state.string_processors.iter_mut().for_each(|processor| {
            processor.bow = Bow {
                velocity: 0.0,
                pressure: 0.0,
            }
        });

        // TODO: Select a string, closest to the last played note that can play the current pitch.
        let note_pitch = Pitch::from(parameters.note_pitch as f64);
        if let Some((string_index, base_pitch)) = self
            .string_pitches
            .iter()
            .enumerate()
            .filter(|(_, base_pitch)| base_pitch.frequency >= note_pitch.frequency)
            .max_by(|(_, a), (_, b)| {
                if a.frequency > b.frequency {
                    Ordering::Greater
                } else if a.frequency < b.frequency {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            })
        {
            let hand_position = calc_hand_position_multiplier(*base_pitch, note_pitch);
            let processor = &mut state.string_processors[string_index];
            
            if parameters.note_off.is_none() {
                // Apply pressure and hand position to the string.
                processor.bow.velocity = 0.1;
                processor.bow.pressure = 10.0 * parameters.gain;
                processor.set_hand_position_multiplier(hand_position);
            }
        }

        // Sample the state of all strings
        state
            .string_processors
            .iter_mut()
            .for_each(|processor| processor.compute_state());
        state
            .string_processors
            .iter_mut()
            .map(|processor| processor.read_output())
            .sum::<f32>()
    }
}
