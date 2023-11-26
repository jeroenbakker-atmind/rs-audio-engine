pub trait Scale {
    type Tones;
    fn tones_per_octave(&self) -> usize;
}
