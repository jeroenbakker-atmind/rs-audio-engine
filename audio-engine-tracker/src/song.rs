use audio_engine_common::{
    beats_per_minute::BeatsPerMinute,
    digital_sound::{parameters::NoteParameters, sound::Sound},
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
    pub instruments: [Instrument; 16],

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
            instruments: [
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
                Instrument::default(),
            ],
        }
    }
}

/// Helper constant to use with #Song::init_patterns when track lengths are not in sync.
pub const _SKIP_ROW__: &str = "";

impl Song {
    pub fn get_phrase(&self, id: PhraseID) -> Option<&Phrase> {
        match id {
            PhraseID::Index(index) => Some(&self.phrases[index as usize]),
            _ => None,
        }
    }
    pub fn get_pattern(&self, id: PatternID) -> Option<&Pattern> {
        match id {
            PatternID::Index(index) => Some(&self.patterns[index as usize]),
            _ => None,
        }
    }

    pub fn get_instrument(&self, id: InstrumentID) -> Option<&Instrument> {
        match id {
            InstrumentID::Index(index) => Some(&self.instruments[index as usize]),
            _ => None,
        }
    }

    /// Initialize multiple patterns in a single call.
    ///
    /// This is useful when you want to keep the rows in several patterns synchronized.
    /// For example when using coords in multiple tracks.
    ///
    /// ```
    /// use audio_engine_tracker::song::Song;
    /// let mut song = Song::default();
    /// song.init_patterns(&[0x00, 0x01, 0x02], &[
    ///     &["C 4 01 FF", "E 4 01 FF", "G 4 01 FF"],
    ///     &["--- -- --", "--- -- --", "--- -- --"],
    ///     &["--- -- --", "--- -- --", "--- -- --"],
    ///     &["--- -- --", "--- -- --", "--- -- --"],
    /// ]);
    /// assert_eq!(4, song.patterns[0x00].count_rows());
    /// assert_eq!(4, song.patterns[0x01].count_rows());
    /// assert_eq!(4, song.patterns[0x02].count_rows());
    /// ```
    ///
    /// Empty rows can be used when tracks don't start at the same beat.
    ///
    /// ```
    /// use audio_engine_tracker::song::{Song, _SKIP_ROW__};
    /// let mut song = Song::default();
    /// song.init_patterns(&[0x00, 0x10, 0x20], &[
    ///     &["C 4 01 FF", "E 4 01 FF", "G 4 01 FF"],
    ///     &["--- -- --", "--- -- --", "--- -- --"],
    ///     &["--- -- --", "--- -- --", "--- -- --"],
    ///     &["--- -- --", "--- -- --", "--- -- --"],
    ///     &[_SKIP_ROW__, "E 4 01 FF", _SKIP_ROW__],
    ///     &[_SKIP_ROW__, "--- -- --", _SKIP_ROW__],
    ///     &[_SKIP_ROW__, "--- -- --", _SKIP_ROW__],
    ///     &[_SKIP_ROW__, "--- -- --", _SKIP_ROW__],
    /// ]);
    /// song.init_patterns(&[0x01, 0x11, 0x21], &[
    ///     &["C 4 01 FF", _SKIP_ROW__, "G 4 01 FF"],
    ///     &["--- -- --", _SKIP_ROW__, "--- -- --"],
    ///     &["--- -- --", _SKIP_ROW__, "--- -- --"],
    ///     &["--- -- --", _SKIP_ROW__, "--- -- --"],
    ///     &["C 4 01 FF", "E 4 01 FF", "G 4 01 FF"],
    ///     &["--- -- --", "--- -- --", "--- -- --"],
    ///     &["--- -- --", "--- -- --", "--- -- --"],
    ///     &["--- -- --", "--- -- --", "--- -- --"],
    /// ]);
    /// assert_eq!(4, song.patterns[0x00].count_rows());
    /// assert_eq!(8, song.patterns[0x01].count_rows());
    /// assert_eq!(8, song.patterns[0x10].count_rows());
    /// assert_eq!(4, song.patterns[0x11].count_rows());
    /// assert_eq!(4, song.patterns[0x20].count_rows());
    /// assert_eq!(8, song.patterns[0x21].count_rows());
    /// ```
    pub fn init_patterns(&mut self, pattern_indices: &[usize], rows: &[&'static [&'static str]]) {
        for (column, pattern_index) in pattern_indices.iter().enumerate() {
            let pattern = &mut self.patterns[*pattern_index];
            let pattern_rows = rows
                .iter()
                .map(|rows| rows[column])
                .filter(|row| !row.is_empty())
                .collect::<Vec<&str>>();
            pattern.init(&pattern_rows);
        }
    }
}

impl Sound for Song {
    type SoundState = SongState;
    type Parameters = NoteParameters;

    fn init_sound_state(&self) -> Self::SoundState {
        let mut state = SongState::default();
        state.init(self);
        state
    }

    fn sample(&self, parameters: &Self::Parameters, state: &mut Self::SoundState) -> f32 {
        todo!("Can tracker call this function?");
        0.0
    }
}
