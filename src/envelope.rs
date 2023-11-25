use crate::{Level, Time};

pub struct Envelope {
    /// Time to delay before the attack starts
    pub delay: Time,
    /// Time to go from 0 to max (1.0)
    pub attack: Time,
    /// Time to hold the max level before decaying.
    pub hold: Time,
    /// Time to decay from max level to sustain.
    pub decay: Time,
    /// Level of the sustain
    pub sustain: Level,
    /// Time after releasing the note.
    pub release: Time,
}

impl Default for Envelope {
    fn default() -> Self {
        Envelope {
            delay: 0.0,
            attack: 0.0,
            hold: 0.0,
            decay: 0.0,
            sustain: 1.0,
            release: 0.0,
        }
    }
}

impl Envelope {
    pub fn level(&self, note_time: Time, note_off: Option<Time>) -> f32 {
        if let Some(note_off) = note_off {
            let value = self.level(note_off, None);
            let interp = (note_time - note_off) / note_off;
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
