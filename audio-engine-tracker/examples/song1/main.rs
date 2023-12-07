use audio_engine_instruments::InstrumentLibrary;
use audio_engine_tracker::{song::Song, song_state::SongState, tracker::Tracker};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> Result<(), ()> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    let song = create_song();

    play_song(&device, &config.into(), song)
}

fn play_song(device: &cpal::Device, config: &cpal::StreamConfig, song: Song) -> Result<(), ()> {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut tracker = Tracker {
        song,
        song_state: SongState::default(),
        frequency: sample_rate,
    };
    println!("Start rendering");
    let samples = tracker.render();
    println!("Finished rendering");
    let song_duration = samples.len() as u64 * 1000 / 44100;

    let mut sample_num = 0;

    let mut next_value = move || {
        sample_num += 1;
        if sample_num >= samples.len() {
            0.0
        } else {
            samples[sample_num]
        }
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            config,
            move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
                for frame in output.chunks_mut(channels) {
                    let value = next_value();
                    for sample in frame.iter_mut() {
                        *sample = value;
                    }
                }
            },
            err_fn,
            None,
        )
        .unwrap();
    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(song_duration));

    Ok(())
}

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
        "--- -- --",
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
