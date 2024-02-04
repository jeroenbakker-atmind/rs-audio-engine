use audio_engine_midinous::song::Song;
use metronome::create_metronome_song;

mod metronome;

pub enum Songs {
    Metronome,
}

impl Songs {
    pub fn create(&self) -> Song {
        match self {
            Songs::Metronome => create_metronome_song(),
        }
    }
}
