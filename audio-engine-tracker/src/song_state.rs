use audio_engine_common::digital_sound::sound_state::SoundState;

use crate::{
    bars_per_beat::BarsPerBeat,
    pattern::PatternID,
    pattern_state::{PatternState, PatternStates},
    phrase::PhraseID,
    phrase_state::{PhraseState, PhraseStates},
    song::Song,
    track_state::TrackState,
};

pub struct SongState {
    pub tracks: [TrackState; 8],
    pub patterns: PatternStates,
    pub phrases: PhraseStates,

    /// Actual rows per beat
    pub rows_per_beat: BarsPerBeat,
}

impl Default for SongState {
    fn default() -> Self {
        SongState {
            tracks: [
                TrackState::default(),
                TrackState::default(),
                TrackState::default(),
                TrackState::default(),
                TrackState::default(),
                TrackState::default(),
                TrackState::default(),
                TrackState::default(),
            ],
            patterns: [PatternState::default(); 255],
            phrases: [PhraseState::default(); 255],
            rows_per_beat: 4.0,
        }
    }
}

impl SongState {
    pub fn init(&mut self, song: &Song) {
        for track in self.tracks.iter_mut() {
            *track = TrackState::default()
        }

        for (pattern, state) in song.patterns.iter().zip(self.patterns.iter_mut()) {
            state.row_len = pattern.count_rows()
        }

        for (phrase, state) in song.phrases.iter().zip(self.phrases.iter_mut()) {
            let mut row_len = 0;
            for pattern_id in phrase.patterns {
                if let Some(pattern) = song.get_pattern(pattern_id) {
                    row_len += pattern.count_rows();
                } else {
                    break;
                }
            }
            state.row_len = row_len;
        }

        self.rows_per_beat = song.initial_speed;
    }

    pub fn get_phrase_row_len(&self, phrase_id: PhraseID) -> u32 {
        if let PhraseID::Index(index) = phrase_id {
            self.phrases[index as usize].row_len
        } else {
            0
        }
    }

    pub fn get_pattern_row_len(&self, pattern_id: PatternID) -> u32 {
        if let PatternID::Index(index) = pattern_id {
            self.patterns[index as usize].row_len
        } else {
            0
        }
    }
}

impl SoundState for SongState {}
