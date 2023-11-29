use crate::{level::Level, note_time::NoteTime};

use super::Envelope;

/// Music Envelope.
///
/// Describes how sound changes over time.
pub struct AttackHoldDecaySustainRelease {
    /// Time to delay before the attack starts
    pub delay: NoteTime,
    /// Time to go from 0 to max (1.0)
    pub attack: NoteTime,
    /// Time to hold the max level before decaying.
    pub hold: NoteTime,
    /// Time to decay from max level to sustain.
    pub decay: NoteTime,
    /// Level of the sustain
    pub sustain: Level,
    /// Time after releasing the note.
    pub release: NoteTime,
}

impl Default for AttackHoldDecaySustainRelease {
    fn default() -> Self {
        AttackHoldDecaySustainRelease {
            delay: 0.0,
            attack: 0.0,
            hold: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
        }
    }
}

impl Envelope for AttackHoldDecaySustainRelease {
    fn level(&self, note_time: NoteTime, note_off: Option<NoteTime>) -> Level {
        if let Some(note_off) = note_off {
            let value = self.level(note_off, None);
            let interp = (note_time - note_off) / self.release;
            (value * (1.0 - interp)).max(0.0)
        } else if note_time < self.delay {
            0.0
        } else if note_time < self.delay + self.attack {
            (note_time - self.delay) / self.attack
        } else if note_time < self.delay + self.attack + self.hold {
            1.0
        } else if note_time < self.delay + self.attack + self.hold + self.decay {
            let decay_time = (note_time - self.delay - self.attack - self.hold) / self.decay;
            decay_time * self.sustain + (1.0 - decay_time)
        } else {
            self.sustain
        }
    }
}
