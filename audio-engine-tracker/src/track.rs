use crate::phrase::PhraseID;

#[derive(Copy, Clone)]
pub struct Track {
    pub phrases: [PhraseID; 255],
}

impl Default for Track {
    fn default() -> Self {
        Self {
            phrases: [PhraseID::default(); 255],
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
