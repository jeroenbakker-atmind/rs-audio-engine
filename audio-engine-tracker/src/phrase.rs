use audio_engine_common::id::ID;

use crate::pattern::PatternID;

#[derive(Default, Copy, Clone)]
pub struct Phrase {
    pub patterns: [PatternID; 16],
}

pub type PhraseID = ID<Phrase>;
