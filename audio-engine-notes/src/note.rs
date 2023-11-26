use std::ops::Add;

use crate::{Octave, Scale, Tone};

pub type NoteStep = i32;

#[derive(Debug, Copy, PartialEq, Clone, PartialOrd)]
pub struct Note<T>
where
    T: Tone + Sized,
{
    pub octave: Octave,
    pub tone: T,
}

impl<T> Note<T>
where
    T: Tone,
    Self: Sized,
{
    pub fn new<I>(tone: I, octave: Octave) -> Self
    where
        I: Into<T>,
    {
        Self {
            tone: tone.into(),
            octave,
        }
    }
}

impl<T> From<i32> for Note<T>
where
    T: Tone + From<u8>,
{
    fn from(value: i32) -> Self {
        let scale = T::scale();
        let octave = value / scale.tones_per_octave() as i32;
        let note_index = value % scale.tones_per_octave() as i32;
        Note::<T>::new(note_index as u8, octave as u8)
    }
}

impl<T> From<Note<T>> for i32
where
    T: Tone,
    u8: From<T>,
{
    fn from(value: Note<T>) -> Self {
        let scale = T::scale();
        (value.octave * scale.tones_per_octave() as u8 + u8::from(value.tone)) as i32
    }
}

impl<T> Add<NoteStep> for Note<T>
where
    T: Tone + From<u8>,
    u8: From<T>,
{
    type Output = Note<T>;
    fn add(self, rhs: NoteStep) -> Self::Output {
        let mut value = i32::from(self);
        value += rhs;
        Note::<T>::from(value)
    }
}
