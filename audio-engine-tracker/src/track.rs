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
