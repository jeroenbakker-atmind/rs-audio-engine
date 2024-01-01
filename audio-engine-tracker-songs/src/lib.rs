use audio_engine_tracker::song::Song;
use songs::{instrument_test::create_instrument_test, song1::create_song1, song2::create_song2};

mod songs;

pub enum SongLibrary {
    Song1,
    Song2,
    InstrumentTest,
}

impl SongLibrary {
    pub fn create(&self) -> Song {
        match self {
            Self::Song1 => create_song1(),
            Self::Song2 => create_song2(),
            Self::InstrumentTest => create_instrument_test(),
        }
    }
}
