use audio_engine_instruments::fm::basic::sine::create_fm_basic_sine_instrument;
use audio_engine_notes::{ChromaticNote, ChromaticTone};
use audio_engine_sequencer::instrument::InstrumentID;
use audio_engine_tracker::{
    event::Event, pattern::PatternID, phrase::PhraseID, row::Row, song::Song,
    song_state::SongState, tracker::Tracker,
};

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() -> Result<(), ()> {
    let host = cpal::default_host();
    let device = host.default_output_device().unwrap();
    let config = device.default_output_config().unwrap();

    play_tone(&device, &config.into())
}

fn play_tone(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), ()> {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    println!("Start rendering");
    let mut tracker = Tracker {
        song: create_song(),
        song_state: SongState::default(),
        frequency: sample_rate,
    };
    let samples = tracker.render();
    println!("Finished rendering");

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

    std::thread::sleep(std::time::Duration::from_millis(10000));

    Ok(())
}

fn create_song() -> Song {
    let mut song = Song::default();

    song.patterns[0].rows[0] = Row {
        event: Some(Event::NoteOn(
            ChromaticNote::new(ChromaticTone::C, 4),
            InstrumentID::from(0),
        )),
        level: Some(0.7),
    };
    song.patterns[0].rows[4] = Row {
        event: Some(Event::NoteOn(
            ChromaticNote::new(ChromaticTone::D, 4),
            InstrumentID::from(0),
        )),
        level: Some(0.5),
    };
    song.patterns[0].rows[8] = Row {
        event: Some(Event::NoteOn(
            ChromaticNote::new(ChromaticTone::E, 4),
            InstrumentID::from(0),
        )),
        level: Some(0.7),
    };
    song.patterns[0].rows[12] = Row {
        event: Some(Event::NoteOn(
            ChromaticNote::new(ChromaticTone::C, 4),
            InstrumentID::from(0),
        )),
        level: Some(0.5),
    };
    song.patterns[0].rows[16] = Row {
        event: Some(Event::PatternEnd),
        level: None,
    };

    song.phrases[0].patterns[0] = PatternID::from(0);
    song.phrases[0].patterns[1] = PatternID::from(0);

    song.tracks[0].phrases[0] = PhraseID::from(0);

    song.instruments[0] = create_fm_basic_sine_instrument();

    song
}
