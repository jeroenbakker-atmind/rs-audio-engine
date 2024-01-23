use audio_engine_notes::ChromaticNote;

#[derive(Debug, Copy, Clone)]
pub struct Arguments {
    pub chromatic_note: ChromaticNote,
    pub buffer_size: usize,
    pub sample_rate: f32,
}
