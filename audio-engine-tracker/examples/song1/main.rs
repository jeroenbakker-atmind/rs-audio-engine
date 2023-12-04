use audio_engine_instruments::fm::basic::sine::create_fm_basic_sine_instrument;
use audio_engine_tracker::{
    pattern::PatternID, phrase::PhraseID, song::Song, song_state::SongState, tracker::Tracker,
};

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
    song.patterns[0].init(&[
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
    song.patterns[1].init(&[
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

    song.patterns[2].init(&[
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

    song.patterns[3].init(&[
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

    song.phrases[0].patterns[0] = PatternID::from(0);
    song.phrases[0].patterns[1] = PatternID::from(0);
    song.phrases[0].patterns[2] = PatternID::from(1);
    song.phrases[0].patterns[3] = PatternID::from(1);

    song.phrases[1].patterns[0] = PatternID::from(2);
    song.phrases[1].patterns[1] = PatternID::from(2);
    song.phrases[1].patterns[2] = PatternID::from(3);
    song.phrases[1].patterns[3] = PatternID::from(3);

    song.tracks[0].phrases[0] = PhraseID::from(0);
    song.tracks[0].phrases[1] = PhraseID::from(1);

    song.instruments[0] = create_fm_basic_sine_instrument();

    song
}
