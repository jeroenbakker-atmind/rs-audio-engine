#[derive(Default, Copy, Clone)]
pub struct PatternState {
    pub row_len: u32,
}

pub type PatternStates = [PatternState; 255];
