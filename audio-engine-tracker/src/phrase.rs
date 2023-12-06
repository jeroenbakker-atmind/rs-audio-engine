use audio_engine_common::id::ID;

use crate::pattern::PatternID;

#[derive(Default, Copy, Clone)]
pub struct Phrase {
    pub patterns: [PatternID; 16],
}

pub type PhraseID = ID<Phrase>;

impl Phrase {
    pub fn init(&mut self, strings: &[&str]) {
        for (string, pattern) in strings.iter().zip(self.patterns.iter_mut()) {
            let pattern_index = hex::decode(string).unwrap()[0];
            *pattern = PatternID::from(pattern_index);
        }
        self.patterns[strings.len()] = PatternID::NotSet;
    }
}
