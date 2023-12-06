use std::fmt::{format, Display, Write};

use audio_engine_common::{id::ID, level::Level};
use audio_engine_notes::{ChromaticNote, ChromaticTone};
use audio_engine_sequencer::instrument::InstrumentID;

use crate::event::Event;

#[derive(Debug, Default, Copy, Clone)]
pub struct Row {
    pub event: Option<Event>,
    pub level: Option<Level>,
}

pub type RowID = ID<Row>;

impl Row {
    pub fn init(&mut self, string: &str) {
        let note_str = &string[0..3];
        let instrument_str = &string[4..6];
        let level_str = &string[7..9];

        self.event = None;
        self.level = None;

        if level_str != "--" {
            self.level = Some(hex::decode(level_str).unwrap()[0] as f32 / 255.0)
        }

        if note_str != "---" {
            if note_str == "REL" {
                self.event = Some(Event::NoteRelease);
            } else if note_str == "OFF" {
                self.event = Some(Event::NoteOff);
            } else {
                let instrument_index = hex::decode(instrument_str).unwrap()[0];
                let instrument_id = InstrumentID::from(instrument_index);

                let tone = if note_str.starts_with("C ") {
                    ChromaticTone::C
                } else if note_str.starts_with("C#") {
                    ChromaticTone::CSharp
                } else if note_str.starts_with("D ") {
                    ChromaticTone::D
                } else if note_str.starts_with("D#") {
                    ChromaticTone::DSharp
                } else if note_str.starts_with("E ") {
                    ChromaticTone::E
                } else if note_str.starts_with("F ") {
                    ChromaticTone::F
                } else if note_str.starts_with("F#") {
                    ChromaticTone::FSharp
                } else if note_str.starts_with("G ") {
                    ChromaticTone::G
                } else if note_str.starts_with("G#") {
                    ChromaticTone::GSharp
                } else if note_str.starts_with("A ") {
                    ChromaticTone::A
                } else if note_str.starts_with("A#") {
                    ChromaticTone::ASharp
                } else if note_str.starts_with("B ") {
                    ChromaticTone::B
                } else {
                    panic!();
                };
                let octave = note_str[2..3].parse::<u8>().unwrap();
                self.event = Some(Event::NoteOn(
                    ChromaticNote::new(tone, octave),
                    instrument_id,
                ));
            }
        }
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.event {
            None | Some(Event::Empty) => {
                f.write_str("--- --")?;
            }
            Some(Event::NoteOn(note, instrument_id)) => {
                match note.tone {
                    ChromaticTone::C => {
                        f.write_str("C ")?;
                    }
                    ChromaticTone::CSharp => {
                        f.write_str("C#")?;
                    }
                    ChromaticTone::D => {
                        f.write_str("D ")?;
                    }
                    ChromaticTone::DSharp => {
                        f.write_str("D#")?;
                    }
                    ChromaticTone::E => {
                        f.write_str("E ")?;
                    }
                    ChromaticTone::F => {
                        f.write_str("F ")?;
                    }
                    ChromaticTone::FSharp => {
                        f.write_str("F#")?;
                    }
                    ChromaticTone::G => {
                        f.write_str("G ")?;
                    }
                    ChromaticTone::GSharp => {
                        f.write_str("G#")?;
                    }
                    ChromaticTone::A => {
                        f.write_str("A ")?;
                    }
                    ChromaticTone::ASharp => {
                        f.write_str("A#")?;
                    }
                    ChromaticTone::B => {
                        f.write_str("B ")?;
                    }
                }
                f.write_fmt(format_args!("{}", note.octave))?;
                f.write_char(' ')?;
                match instrument_id {
                    InstrumentID::_PhantomData(_) | InstrumentID::NotSet => {
                        f.write_str("--")?;
                    }
                    InstrumentID::Index(index) => {
                        f.write_fmt(format_args!("{:02}", &hex::encode_upper(&[index])))?;
                    }
                }
            }
            Some(Event::NoteRelease) => {
                f.write_str("REL --")?;
            }
            Some(Event::NoteOff) => {
                f.write_str("OFF --")?;
            }
            Some(Event::PatternEnd) => {
                f.write_str("END   ")?;
            }
        }

        f.write_char(' ')?;
        match self.level {
            None => {
                f.write_str("--")?;
            }
            Some(level) => {
                let level_u8 = (level * 255.0) as u8;
                f.write_fmt(format_args!("{:02}", &hex::encode_upper([level_u8])))?;
            }
        }

        Ok(())
    }
}
