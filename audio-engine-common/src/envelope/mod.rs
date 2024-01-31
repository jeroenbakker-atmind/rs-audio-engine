use crate::{level::Level, note_time::NoteTime};

pub mod delay_attack_decay_sustain_release;
pub mod delay_attack_hold_decay_sustain_release;
pub mod trapezoid;

pub trait Envelope {
    fn level(&self, note_time: NoteTime, note_off: Option<NoteTime>) -> Level;
}
