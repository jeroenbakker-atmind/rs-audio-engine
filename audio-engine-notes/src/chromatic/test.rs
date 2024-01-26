use crate::{ChromaticNote, Pitch};

/// Test that converts each chromatic note into a pitch and back. The chromatic notes should match.
#[test]
fn to_pitch_and_back() {
    for octave in 0..8 {
        for tone in 0..12 {
            let note_before = ChromaticNote::new(tone, octave);
            let pitch = note_before.pitch();
            let note_after = ChromaticNote::from(pitch);
            assert_eq!(note_before, note_after);
        }
    }
}