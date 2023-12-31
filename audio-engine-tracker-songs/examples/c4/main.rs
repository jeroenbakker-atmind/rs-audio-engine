use audio_engine_tracker::{song::Song, song_state::SongState, tracker::Tracker};

use audio_engine_tracker_songs::SongLibrary;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> Result<(), ()> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    let song = SongLibrary::C4.create();

    play_song(&device, &config.into(), song)
}

fn play_song(device: &cpal::Device, config: &cpal::StreamConfig, song: Song) -> Result<(), ()> {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut tracker = Tracker {
        song,
        song_state: SongState::default(),
        sample_rate,
    };
    println!("Start rendering");
    let samples = tracker.render();
    println!("Finished rendering");
    let song_duration = samples.len() as u64 * 1000 / sample_rate as u64;

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
