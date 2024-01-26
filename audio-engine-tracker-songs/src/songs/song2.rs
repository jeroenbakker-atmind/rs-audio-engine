use audio_engine_instruments::InstrumentLibrary;
use audio_engine_tracker::{
    phrase::PhraseID,
    song::{Song, _SKIP_ROW__},
    song_state::SongState,
};

pub fn create_song2() -> Song {
    let mut song = Song {
        speed: 130.0,
        initial_speed: 4.0,
        ..Song::default()
    };

    /* Track 0-3: Piano */
    song.init_patterns(
        &[0x00, 0x80],
        &[
            &["OFF -- --", "E 2 01 FF"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["G 3 00 80", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["B 3 -- 80", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["E 4 -- A0", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["B 3 -- 80", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["G 3 -- 80", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
        ],
    );

    song.init_patterns(
        &[0x01, 0x81],
        &[
            &["OFF -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["G 3 -- 80", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["B 3 -- 80", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["E 4 -- A0", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["B 3 -- 80", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["B 4 -- FF", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["--- -- --", _SKIP_ROW__],
            &["B 4 -- FF", "E 2 -- FF"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "B 2 -- 80"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "E 3 -- 80"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["B 4 -- FF", "G 3 -- A0"],
            &["--- -- --", "--- -- --"],
            &["E 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "E 3 -- 80"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "B 2 -- 80"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "E 2 -- FF"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "B 2 -- 80"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["B 4 -- FF", "E 3 -- 80"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["C 5 -- FF", "G 3 -- A0"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["B 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["C 5 -- FF", "--- -- --"],
            &["B 4 -- FF", "--- -- --"],
            &["A 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["B 4 -- FF", "B 3 -- FF"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["A 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["G 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["E 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["E 4 -- FF", "A 2 -- FF"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["C 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["E 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["F#4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["E 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["E 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["C 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --"],
        ],
    );
    song.patterns[0x02].init(&[
        "OFF -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 4 -- FF",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
        "C 4 -- FF",
        "--- -- --",
        "G 4 -- FF",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
        "C 4 -- FF",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "F#4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "D 4 -- FF",
        "--- -- --",
        "F#4 -- FF",
        "--- -- --",
        "D 4 -- FF",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
        "A 4 -- FF",
        "E 4 -- FF",
        "F#4 -- FF",
        "--- -- --",
        "B 3 -- FF",
        "--- -- --",
        "B 3 -- FF",
        "--- -- --",
        "E 4 -- FF",
        "--- -- --",
    ]);

    song.patterns[0x03].init(&[
        "OFF -- --",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "E 5 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "G 4 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
    ]);

    song.init_patterns(
        &[0x04, 0x21, 0x83],
        &[
            &["OFF -- --", "OFF -- --", "E 2 -- FF"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "E 3 -- FF"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["B 3 -- FF", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["E 4 -- FF", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["G 4 -- FF", "B 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["OFF -- --", "OFF -- --", "D 3 -- FF"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["A 3 -- FF", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["D 4 -- FF", "F#4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["OFF -- --", "OFF -- --", "C 3 -- FF"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["G 3 -- FF", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["C 4 -- FF", "E 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
        ],
    );
    song.init_patterns(
        &[0x05, 0x22, 0x84],
        &[
            &["OFF -- --", "OFF -- --", "G 2 -- FF"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["G 3 -- FF", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["B 3 -- FF", "D 4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["OFF -- --", "OFF -- --", "B 2 -- FF"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["F#3 -- FF", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["B 3 -- FF", "D#4 -- FF", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
            &["--- -- --", "--- -- --", "--- -- --"],
        ],
    );

    /* Track 4-6: Base */
    song.patterns[0x82].init(&[
        "A 2 -- FF",
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
        "A 2 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "B 2 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 3 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "C 3 -- FF",
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
        "C 3 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "D 3 -- FF",
        "--- -- --",
        "--- -- --",
        "--- -- --",
        "D 3 -- FF",
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

    /* Couplet 1
     *
     * So close, no matter how far
     * Couldn't be much more from the heart
     * Forever trusting who we are
     * And nothing else matters.
     */
    song.init_patterns(
        &[0x06],
        &[
            // Measure
            &["OFF -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            // Measure
            &["F#4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            // Measure
            &["OFF -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            // Measure
            &["D 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["D 4 -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            // Measure
            &["OFF -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["G 4 -- --"],
            &["--- -- --"],
            &["G 4 -- --"],
            &["--- -- --"],
            &["G 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["G 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["B 3 -- --"],
            &["--- -- --"],
            // Measure
            &["D 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["D 4 -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["D 4 -- --"],
            &["--- -- --"],
            &["C 4 -- --"],
            &["--- -- --"],
            &["C 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            // Measure
            &["OFF -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["G 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["F#4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["F#4 -- --"],
            &["--- -- --"],
            &["G 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            // Measure
            &["F#4 -- --"],
            &["--- -- --"],
            &["E 4 -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            // Measure
            &["OFF -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
        ],
    );
    song.init_patterns(
        &[ 0x85],
        &[
            &["E 2 -- FF"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["B 2 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 3 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["G 3 -- A0"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 3 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["B 2 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
        ]
    );
    song.init_patterns(
        &[ 0x86],
        &[
            &["D 2 -- FF"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["A 2 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["D 3 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["C 2 -- A0"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["C 3 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["G 2 -- 80"],
            &["--- -- --"],
            &["F#2 -- 60"],
            &["--- -- --"],
        ]
    );
    song.init_patterns(
        &[ 0x87],
        &[

            // Measure
            &["D 2 -- FF"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["A 2 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["D 3 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["C 2 -- A0"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 2 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["C 3 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            // Measure
            &["G 2 -- FF"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["D 3 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["G 3 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["B 2 -- A0"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["E 3 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
            &["F#3 -- 80"],
            &["--- -- --"],
            &["--- -- --"],
            &["--- -- --"],
        ],
    );

    /* Utility */
    song.patterns[0xfe].init(&["--- -- --"; 24]);
    song.patterns[0xfd].init(&["--- -- --"; 108]);

    song.phrases[0x00].init(&["00", "00", "00", "01", "02", "03", "00", "00", "00"]);
    song.phrases[0x20].init(&[
        "FE", "FE", "FE", "FD", "FE", "FE", "FE", "FE", "FE", "FE", "FE",
    ]);
    song.phrases[0x80].init(&["80", "80", "80", "80", "81", "82", "80", "80", "80", "80"]);

    song.phrases[0x01].init(&["04", "04", "04", "05", "00", "00"]);
    song.phrases[0x21].init(&["21", "21", "21", "22", "FE", "FE"]);
    song.phrases[0x81].init(&["83", "83", "83", "84", "80", "80"]);

    song.phrases[0x02].init(&["06"]);
    song.phrases[0x22].init(&["06"]);
    song.phrases[0x82].init(&["85", "86", "85", "86", "85", "87", "85", "85"]);

    song.phrases[0xFE].init(&["FE", "FE", "FE", "FE"]);

    song.tracks[0x00].init(&["00", "01", "02"]);
    song.tracks[0x00].level = 0.6;
    song.tracks[0x01].init(&["20", "21", "22"]);
    song.tracks[0x01].level = 0.6;
    song.tracks[0x04].init(&["80", "81", "82"]);
    song.tracks[0x04].level = 0.6;

    song.instruments[0] = InstrumentLibrary::PianoPiano.create();
    song.instruments[1] = InstrumentLibrary::PianoPiano.create();

    {
        let mut song_state = SongState::default();
        song_state.init(&song);
        assert_eq!(
            song_state.get_phrase_row_len(PhraseID::from(0x00)),
            song_state.get_phrase_row_len(PhraseID::from(0x20))
        );
        assert_eq!(
            song_state.get_phrase_row_len(PhraseID::from(0x00)),
            song_state.get_phrase_row_len(PhraseID::from(0x80))
        );
    }

    song
}
