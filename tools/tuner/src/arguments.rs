use audio_engine_notes::ChromaticNote;
use clap::Parser;

#[derive(Debug, Copy, Clone, Parser)]
#[command(
    about = "Instrument tuner",
    long_about = "Tool for tuning instruments by listening to an audio source and determine the pitch with the most amplitude."
)]
pub struct Arguments {
    /// Note to tune towards.
    ///
    /// Note is specified in chromatic scale.
    /// Example values are (C2, F#4, etc)
    pub note: ChromaticNote,

    /// Buffer size for recording.
    #[arg(long, default_value_t = 4096)]
    pub buffer_size: usize,

    /// Number of steps to use for detecting the pitch of the recorded audio.
    ///
    /// The higher the number the more steps are used, but would lead to lower
    /// performance.
    #[arg(long, default_value_t = 4096)]
    pub steps: usize,

    /// Minimum threshold of the volume to start tuning
    #[arg(long, default_value_t = 0.001)]
    pub threshold: f32,
}

impl Arguments {}
