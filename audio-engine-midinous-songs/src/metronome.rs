use audio_engine_instruments::InstrumentLibrary;
use audio_engine_midinous::{
    builder::Builder, builder_link::LinkBuilder, builder_node::NodeBuilder,
    builder_song::SongBuilder, song::Song,
};
use audio_engine_notes::{ChromaticNote, ChromaticTone};

pub fn create_metronome_song() -> Song {
    SongBuilder::new()
        .instrument(InstrumentLibrary::PianoPiano)
        .entry_point(
            *NodeBuilder::new()
                .location(-2.0, 2.0)
                .note_pitch(ChromaticNote::new(ChromaticTone::G, 4).pitch())
                .note_level(1.0),
        )
        .connect_to_last(
            *NodeBuilder::new()
                .location(2.0, 2.0)
                .note_pitch(ChromaticNote::new(ChromaticTone::C, 4).pitch())
                .note_level(0.6),
        )
        .connect_to_last(
            *NodeBuilder::new()
                .location(2.0, -2.0)
                .note_pitch(ChromaticNote::new(ChromaticTone::C, 4).pitch())
                .note_level(0.6),
        )
        .connect_to_last(
            *NodeBuilder::new()
                .location(-2.0, -2.0)
                .note_pitch(ChromaticNote::new(ChromaticTone::C, 4).pitch())
                .note_level(0.6),
        )
        .link(*LinkBuilder::new().from_node(3).to_node(0))
        .build()
}
