use std::cmp::Ordering;

use audio_engine_common::digital_sound::{parameters::NoteParameters, sound::Sound};
use audio_engine_notes::Pitch;

use crate::{
    bow::Bow,
    instrument_state::BowedStringInstrumentState,
    modal_processor::ModalProcessor,
    processor::StringProcessor,
    string::{calc_hand_position_multiplier, String},
};

/// For testing purposes should we process each string, or only the active string.
/// Still requires more testing the goal is to emulate strings/bow transition.
/// for performance reasons we might only want to evaluate strings that have been played
/// recently.
const READ_ALL_STRINGS: bool = false;

/// Output state changes to the console for debugging purposes.
const DEBUG_STATE_CHANGES: bool = false;

#[derive(Debug, Default, Clone)]
pub struct BowedStringInstrument {
    pub strings: Vec<String>,
    /// The base pitch for each string in strings attribute.
    pub string_pitches: Vec<Pitch>,
    // TODO: Add bow velocity envelope
    // TODO: Add bow pressure envelope
}

impl BowedStringInstrument {
    pub fn add_string<P>(&mut self, string: String, string_pitch: P)
    where
        P: Into<Pitch> + Sized,
    {
        self.strings.push(string);
        self.string_pitches.push(string_pitch.into())
    }

    fn init_processors(&self, sample_rate: f64, state: &mut BowedStringInstrumentState) {
        for string in &self.strings {
            let mut processor = ModalProcessor::new(sample_rate, string);
            processor.gain = 1000.0;
            state.string_processors.push(processor);
        }
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
            self.init_processors(parameters.sample_rate as f64, state);
        }

        let is_new_note = state.last_note_time > parameters.note_time;
        if is_new_note {
            // Remove pressure from all strings
            state.string_processors.iter_mut().for_each(|processor| {
                processor.reset_string_states();
                processor.bow = Bow {
                    velocity: 0.0,
                    pressure: 0.0,
                }
            });
        }
        state.last_note_time = parameters.note_time;

        // TODO: Select a string, closest to the last played note that can play the current pitch.
        // TODO: We should introduce playing styles
        let note_pitch = Pitch::from(parameters.note_pitch as f64);
        if let Some((string_index, base_pitch)) = self
            .string_pitches
            .iter()
            .enumerate()
            .filter(|(_, base_pitch)| base_pitch.frequency < note_pitch.frequency)
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
            if state.last_string_index != string_index {
                if DEBUG_STATE_CHANGES {
                    println!(
                        "string_altered(previous={},new={string_index})",
                        state.last_string_index
                    );
                }

                state.last_string_index = string_index;
                state.last_hand_position = 100.0;
            }

            let hand_position = calc_hand_position_multiplier(*base_pitch, note_pitch);
            if state.last_hand_position != hand_position {
                if DEBUG_STATE_CHANGES {
                    println!(
                        "position_altered(previous={},new={hand_position})",
                        state.last_hand_position
                    );
                }
                state.last_hand_position = hand_position;
                let processor = &mut state.string_processors[string_index];
                processor.set_hand_position_multiplier(hand_position);
            }

            let processor = &mut state.string_processors[string_index];
            if parameters.note_off.is_none() {
                // Apply pressure and hand position to the string.
                processor.bow.velocity = 0.2;
                processor.bow.pressure = 10.0 * parameters.gain as f64;
            }
        }

        // Sample the strings
        let result = if READ_ALL_STRINGS {
            state
                .string_processors
                .iter_mut()
                .map(|processor| processor.read_output())
                .sum::<f64>()
        } else {
            let processor = &mut state.string_processors[state.last_string_index];
            processor.read_output()
        };
        result as f32
    }
}
