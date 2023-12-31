use audio_engine_instruments::InstrumentLibrary;
use audio_engine_tracker::song::Song;

pub fn create_song2() -> Song {
    let mut song = Song {
        speed: 120.0,
        initial_speed: 4.0,
        ..Song::default()
    };

    /* Track 0-3: Piano */
    song.patterns[0x00].init(&[
        "OFF -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 3 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 3 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 3 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 3 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);

    song.patterns[0x01].init(&[
        "OFF -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 3 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 3 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 3 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 4 00 FF",
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
        "B 4 00 FF",
        "--- -- --",
        "E 4 00 FF",
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
        "--- -- --",
        "B 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 5 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 4 00 FF",
        "--- -- --",
        "C 5 00 FF",
        "B 4 00 FF",
        "A 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "A 4 00 FF",
        "--- -- --",
        "G 4 00 FF",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "F#4 00 FF",
        "--- -- --",
        "E 4 00 FF",
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
    song.patterns[0x02].init(&[
        "OFF -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "G 4 00 FF",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "C 4 00 FF",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "F#4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "D 4 00 FF",
        "--- -- --",
        "F#4 00 FF",
        "--- -- --",
        "D 4 00 FF",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
        "A 4 00 FF",
        "E 4 00 FF",
        "F#4 00 FF",
        "--- -- --",
        "B 3 00 FF",
        "--- -- --",
        "B 3 00 FF",
        "--- -- --",
        "E 4 00 FF",
        "--- -- --",
    ]);

    song.patterns[0x03].init(&[
        "OFF -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 5 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 4 00 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);

    /* Track 4-6: Base */
    song.patterns[0x80].init(&[
        "E 2 01 FF",
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
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);

    song.patterns[0x81].init(&[
        "E 2 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 2 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 3 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 3 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 3 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 2 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 2 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 2 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 3 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 3 01 FF",
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
        "B 3 01 FF",
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
        "A 3 01 FF",
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
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);
    song.patterns[0x82].init(&[
        "A 2 01 FF",
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
        "A 2 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 2 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 3 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 3 01 FF",
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
        "C 3 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "D 3 01 FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "D 3 01 FF",
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
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);

    /* Utility */
    song.patterns[0xfe].init(&["--- -- --"; 24]);
    song.patterns[0xfd].init(&["--- -- --"; 108]);

    song.phrases[0x00].init(&["00", "00", "00", "01", "02", "03", "00", "00", "00"]);
    song.phrases[0x80].init(&["80", "80", "80", "80", "81", "82", "80", "80", "80", "80"]);
    song.phrases[0xFE].init(&["FE", "FE", "FE", "FE"]);

    song.tracks[0x00].init(&["00"]);
    song.tracks[0x00].level = 0.6;
    song.tracks[0x04].init(&["80"]);
    song.tracks[0x04].level = 0.6;

    song.instruments[0] = InstrumentLibrary::PianoPiano.create();
    song.instruments[1] = InstrumentLibrary::PianoPiano.create();

    song
}
