use crate::{level::Level, note_time::NoteTime};

use super::Envelope;

/// Envelope in the shape of a trapeziod.
/// 
/// ```txt
///   D----------D
///  /            \
/// C-------------E
/// 
/// <A>         <B>
/// 
/// A: attack time
/// B: releae time
/// C: start level
/// D: hold level
/// E: release level
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Trapezoid {
    /// Level at the start of the envelope (note_time = 0.0)
    pub start: Level,
    /// Time to get from the start to the hold level.
    pub attack: NoteTime,
    /// Level to hold between the attack and release.
    pub hold: Level,
    /// Time that the note is still audible after release. Level will be interpolated from end
    pub release: NoteTime,
    /// Level when note is fully released.
    pub end: Level,
}

impl Default for Trapezoid {
    fn default() -> Self {
        Self {
            start: 0.0,
            attack: 0.2,
            hold: 1.0,
            release: 1.0,
            end: 0.0,
        }
    }
}

impl Envelope for Trapezoid {
    fn level(
        &self,
        note_time: crate::note_time::NoteTime,
        note_off: Option<crate::note_time::NoteTime>,
    ) -> crate::level::Level {
        if let Some(note_off) = note_off {
            let release_time = note_time - note_off;
            let release_level = self.level(note_time, None);
            if release_time > self.release {
                self.end
            } else if release_time < 0.0 {
                release_level
            } else {
                let interp = release_time / self.release;
                release_level * (1.0 - interp) + self.end * interp
            }
        } else if note_time < self.attack {
            let interp = note_time / self.attack;
            self.start * (1.0 - interp) + self.hold * interp
        } else {
            self.hold
        }
    }
}
