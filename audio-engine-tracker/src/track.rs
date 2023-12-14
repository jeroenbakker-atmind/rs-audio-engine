use audio_engine_common::level::Level;
use audio_engine_effect_delay::delay::Delay;
use audio_engine_effect_distortion::distortion::Distortion;

use crate::phrase::PhraseID;

#[derive(Copy, Clone)]
pub struct Track {
    pub level: Level,
    pub phrases: [PhraseID; 255],

    pub delay: Delay,
    pub distortion: Distortion,
}

impl Default for Track {
    fn default() -> Self {
        Self {
            level: 1.0,
            phrases: [PhraseID::default(); 255],
            delay: Delay::default(),
            distortion: Distortion::default(),
        }
    }
}

impl Track {
    pub fn init(&mut self, strings: &[&str]) {
        for (string, phrase) in strings.iter().zip(self.phrases.iter_mut()) {
            let pattern_index = hex::decode(string).unwrap()[0];
            *phrase = PhraseID::from(pattern_index);
        }
        self.phrases[strings.len()] = PhraseID::NotSet;
    }
}
