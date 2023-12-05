#[derive(Default, Copy, Clone)]
pub struct PhraseState {
    pub row_len: u32,
}

pub type PhraseStates = [PhraseState; 255];
