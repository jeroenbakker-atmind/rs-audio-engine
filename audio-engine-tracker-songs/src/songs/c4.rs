use audio_engine_instruments::InstrumentLibrary;
use audio_engine_tracker::song::Song;

pub fn create_c4() -> Song {
    let mut song = Song {
        speed: 120.0,
        ..Song::default()
    };
    song.patterns[0x00].init(&[
        "C 4 00 FF",
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
    song.phrases[0x00].init(&["00", "00", "00", "00"]);

    song.tracks[0x00].init(&["00"]);
    song.tracks[0x00].level = 1.0;

    song.instruments[0] = InstrumentLibrary::SamplePianosPianoAX.create();

    song
}
