use audio_engine_instruments::InstrumentLibrary;
use audio_engine_tracker::song::Song;

pub fn create_instrument_test() -> Song {
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
    song.patterns[0x01].init(&[
        "C 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
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
        "--- -- --",
    ]);
    song.patterns[0x02].init(&[
        "C 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 4 00 FF",
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
        "C 4 00 FF",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
    ]);
    song.patterns[0x04].init(&[
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
        "C 4 00 FF",
    ]);
    song.patterns[0x10].init(&[
        "C 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "D 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "F 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);
    song.phrases[0x00].init(&["00", "01", "02", "03", "04"]);
    song.phrases[0x01].init(&["10", "10", "10"]);

    song.tracks[0x00].init(&["00", "01"]);
    song.tracks[0x00].level = 1.0;

    song.instruments[0] = InstrumentLibrary::PianoPiano.create();

    song
}
