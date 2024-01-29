use crate::{level::Level, note_time::NoteTime};

pub trait SoundParameters {
    fn get_sample_rate(&self) -> f32;
}

/// Default Parameters for digital sound.
pub struct NoteParameters {
    /// Time in seconds from the start of playing the note.
    pub note_time: NoteTime,
    /// Time in seconds from the start of playing the note, when the note was released.
    pub note_off: Option<NoteTime>,
    /// Pitch of the note being played.
    pub note_pitch: f32,
    /// Current gain (volume/amplification level)
    pub gain: Level,
    /// Sample rate being used to sample the Sound.
    pub sample_rate: f32,
}

impl SoundParameters for NoteParameters {
    fn get_sample_rate(&self) -> f32 {
        self.sample_rate
    }
}
