use crate::note_time::NoteTime;

pub trait SoundParameters {}

/// Default Parameters for digital sound.
pub struct NoteParameters {
    /// Time in seconds from the start of playing the note.
    pub note_time: NoteTime,
    /// Time in seconds from the start of playing the note, when the note was released.
    pub note_off: Option<NoteTime>,
    /// Pitch of the note being played.
    pub note_pitch: f32,
    /// Sample rate being used to sample the Sound.
    pub sample_rate: f32,
}

impl SoundParameters for NoteParameters {}
