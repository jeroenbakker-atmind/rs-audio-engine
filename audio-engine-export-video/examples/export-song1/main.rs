use audio_engine_export_video::export::export_audio_to_file;
use audio_engine_instruments::InstrumentLibrary;
use audio_engine_tracker::{song::Song, song_state::SongState, tracker::Tracker};

fn main() {
    let song = create_song();
    let sample_rate = 44100.0;

    let mut tracker = Tracker {
        song,
        song_state: SongState::default(),
        sample_rate,
    };

    println!("Begin rendering");
    let samples = tracker.render();
    println!("Finished rendering");

    println!("Begin exporting");
    export_audio_to_file("", &samples, sample_rate);
    println!("Finished exporting");
}

// TODO: move to audio-engine-songs

fn create_song() -> Song {
    let mut song = Song {
        speed: 136.0,
        ..Song::default()
    };
    song.patterns[0x00].init(&[
        "C 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "D 4 00 80",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 4 00 80",
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);
    song.patterns[0x01].init(&[
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "F 4 00 80",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);

    song.patterns[0x02].init(&[
        "G 4 00 FF",
        "--- -- --",
        "A 4 00 80",
        "--- -- --",
        "G 4 00 80",
        "--- -- --",
        "F 4 00 80",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);

    song.patterns[0x03].init(&[
        "C 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 3 00 80",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "OFF -- --",
    ]);

    song.patterns[0xfe].init(&[
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);

    song.phrases[0x00].init(&["00", "00", "01", "01"]);
    song.phrases[0x01].init(&["02", "02", "03", "03"]);
    song.phrases[0xFE].init(&["FE", "FE"]);

    song.tracks[0x00].init(&["00", "01"]);
    song.tracks[0x01].init(&["FE", "00", "01"]);
    song.tracks[0x00].level = 0.6;
    song.tracks[0x01].level = 0.4;

    song.instruments[0] = InstrumentLibrary::FmWIP.create();

    song
}
