use audio_engine_tracker::song::Song;
use songs::{c4::create_c4, song1::create_song1, song2::create_song2};

mod songs;

pub enum SongLibrary {
    Song1,
    Song2,
    C4,
}

impl SongLibrary {
    pub fn create(&self) -> Song {
        match self {
            Self::Song1 => create_song1(),
            Self::Song2 => create_song2(),
            Self::C4 => create_c4(),
        }
    }
}
