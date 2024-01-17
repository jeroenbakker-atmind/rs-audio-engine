/// The hand that doesn't hold the bow can block the string to change the pitch.
#[derive(Debug)]
pub struct Hand {
    /// 1.0 = hand doesn't block the string,
    /// 0.5 = hand blocks string halfway (one octave up)
    pub fretting_position: f32,
}

impl Default for Hand {
    fn default() -> Self {
        Hand {
            fretting_position: 1.0,
        }
    }
}
