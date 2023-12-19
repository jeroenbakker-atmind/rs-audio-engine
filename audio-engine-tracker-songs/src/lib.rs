use audio_engine_tracker::song::Song;
use songs::song1::create_song1;

mod songs;

pub enum SongLibrary {
    Song1,
}

impl SongLibrary {
    pub fn create(&self) -> Song {
        match self {
            Self::Song1 => create_song1(),
        }
    }
}
