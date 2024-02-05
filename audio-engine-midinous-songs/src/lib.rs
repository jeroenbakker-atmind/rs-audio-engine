use audio_engine_midinous::song::Song;
use metronome::create_metronome_song;

mod metronome;

pub enum SongLibrary {
    Metronome,
}

impl SongLibrary {
    pub fn create(&self) -> Song {
        match self {
            SongLibrary::Metronome => create_metronome_song(),
        }
    }
}
