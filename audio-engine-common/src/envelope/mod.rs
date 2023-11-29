use crate::{level::Level, note_time::NoteTime};

pub mod attack_decay_sustain_release;
pub mod attack_hold_decay_sustain_release;

pub trait Envelope {
    fn level(&self, note_time: NoteTime, note_off: Option<NoteTime>) -> Level;
}
