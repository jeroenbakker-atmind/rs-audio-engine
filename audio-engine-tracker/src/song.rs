use audio_engine_common::{
    beats_per_minute::BeatsPerMinute, digital_sound::sound::Sound, id::GetID,
};
use audio_engine_sequencer::instrument::{Instrument, InstrumentID};

use crate::{
    bars_per_beat::BarsPerBeat,
    pattern::{Pattern, PatternID},
    phrase::{Phrase, PhraseID},
    song_state::SongState,
    track::Track,
};

pub struct Song {
    pub speed: BeatsPerMinute,
    pub tracks: [Track; 8],
    pub patterns: [Pattern; 255],
    pub phrases: [Phrase; 255],
    pub instruments: [Instrument; 255],

    /// Initial speed. Speed is the number of rows that will be played per beat.
    pub initial_speed: BarsPerBeat,
}

impl Default for Song {
    fn default() -> Self {
        Self {
            speed: 120.0,
            initial_speed: 4.0,
            tracks: [Track::default(); 8],
            patterns: [Pattern::default(); 255],
            phrases: [Phrase::default(); 255],
            instruments: [Instrument::default(); 255],
        }
    }
}

impl GetID<Phrase> for Song {
    fn get(&self, id: PhraseID) -> Option<&Phrase> {
        match id {
            PhraseID::Index(index) => Some(&self.phrases[index as usize]),
            _ => None,
        }
    }
}

impl GetID<Pattern> for Song {
    fn get(&self, id: PatternID) -> Option<&Pattern> {
        match id {
            PatternID::Index(index) => Some(&self.patterns[index as usize]),
            _ => None,
        }
    }
}

impl GetID<Instrument> for Song {
    fn get(&self, id: InstrumentID) -> Option<&Instrument> {
        match id {
            InstrumentID::Index(index) => Some(&self.instruments[index as usize]),
            _ => None,
        }
    }
}

impl Sound for Song {
    type SoundState = SongState;

    fn init_sound_state(&self) -> Self::SoundState {
        let mut state = SongState::default();
        state.init(self);
        state
    }

    fn sample(
            &self,
            note_time: audio_engine_common::note_time::NoteTime,
            note_off: Option<audio_engine_common::note_time::NoteTime>,
            note_pitch: f32,
            sample_rate: f32,
            state: &mut Self::SoundState,
        ) -> f32 {
        0.0
    }
}
