use std::{cmp::Ordering, marker::PhantomData};

use audio_engine_common::{
    digital_sound::{parameters::NoteParameters, sound::Sound},
    envelope::{trapezoid::Trapezoid, Envelope},
};
use audio_engine_notes::Pitch;

use crate::{
    bow::Bow,
    instrument_state::BowedStringInstrumentState,
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

#[derive(Debug, Clone)]
pub struct BowedStringInstrument<P> {
    pub strings: Vec<String>,
    /// The base pitch for each string in strings attribute.
    pub string_pitches: Vec<Pitch>,
    // Velocity envelope. Actual velocity should be included in the envelope in meters per second.
    pub velocity_envelope: Trapezoid,
    // TODO: Add bow pressure envelope
    _processor: PhantomData<P>,
}

impl<P> Default for BowedStringInstrument<P> {
    fn default() -> Self {
        Self {
            strings: Default::default(),
            string_pitches: Default::default(),
            velocity_envelope: Trapezoid {
                start: 0.1,
                attack: 0.2,
                hold: 0.2,
                release: 0.5,
                end: 0.05,
            },
            _processor: Default::default(),
        }
    }
}

impl<P> BowedStringInstrument<P>
where
    P: StringProcessor + Sized + Clone,
{
    pub fn add_string<PI>(&mut self, string: String, string_pitch: PI)
    where
        PI: Into<Pitch> + Sized,
    {
        self.strings.push(string);
        self.string_pitches.push(string_pitch.into())
    }

    fn init_processors(&self, sample_rate: f64, state: &mut BowedStringInstrumentState<P>) {
        for string in &self.strings {
            let mut processor = P::new(sample_rate, string);
            processor.set_gain(1000.0);
            state.string_processors.push(processor);
        }
    }
}

impl<P> Sound for BowedStringInstrument<P>
where
    P: StringProcessor + Clone + Sized,
{
    type SoundState = BowedStringInstrumentState<P>;
    type Parameters = NoteParameters;

    fn init_sound_state(&self) -> Self::SoundState {
        BowedStringInstrumentState::default()
    }

    fn sample(&self, parameters: &Self::Parameters, state: &mut Self::SoundState) -> f32 {
        if state.string_processors.is_empty() {
            self.init_processors(parameters.sample_rate as f64, state);
        }

        let is_new_note = state.last_note_time > parameters.note_time;
        if is_new_note {
            // Remove pressure from all strings
            state.string_processors.iter_mut().for_each(|processor| {
                processor.reset_string_states();
                processor.update_bow(Bow {
                    velocity: 0.0,
                    pressure: 0.0,
                });
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
            /*  Use not larger then when selecting string include equal frequencies. */
            .filter(|(_, base_pitch)| !(base_pitch.frequency > note_pitch.frequency))
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
                processor.update_bow(Bow {
                    pressure: 10.0 * parameters.gain as f64,
                    velocity: self
                        .velocity_envelope
                        .level(parameters.note_time, parameters.note_off)
                        as f64,
                });
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
