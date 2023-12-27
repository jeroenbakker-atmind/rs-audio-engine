use audio_engine_export_video::{export::export_audio_to_video, types::VideoType};
use audio_engine_tracker::{song_state::SongState, tracker::Tracker};
use audio_engine_tracker_songs::SongLibrary;

fn main() {
    let song = SongLibrary::Song1.create();
    let sample_rate = 44100.0;

    let mut tracker = Tracker {
        song,
        song_state: SongState::default(),
        sample_rate: 44100.0,
    };

    println!("Begin rendering");
    let samples = tracker.render();
    println!("Finished rendering");

    println!("Begin exporting");
    export_audio_to_video(
        "export-song1-histogram.mp4",
        &samples,
        sample_rate,
        VideoType::Histogram,
    );
    println!("Finished exporting");
}
