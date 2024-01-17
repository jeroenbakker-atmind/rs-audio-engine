#[derive(Debug, Default)]
pub struct Bow {
    /// Bow Pressure (fb)
    pub pressure: f32,
    /// Bow Speed in meters per second (vb/bowVel)
    pub velocity: f32,
}

pub const BOW_FREE_PARAMETER: f32 = 100.0;
