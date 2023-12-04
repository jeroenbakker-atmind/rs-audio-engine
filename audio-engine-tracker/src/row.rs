use audio_engine_common::{id::ID, level::Level};
use audio_engine_notes::{ChromaticNote, ChromaticTone};
use audio_engine_sequencer::instrument::InstrumentID;

use crate::event::Event;

#[derive(Default, Copy, Clone)]
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
