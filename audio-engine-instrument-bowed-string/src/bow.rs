#[derive(Debug, Default, Copy, Clone)]
pub struct Bow {
    /// Bow Pressure (fb)
    pub pressure: f64,
    /// Bow Speed in meters per second (vb/bowVel)
    pub velocity: f64,
}

pub const BOW_FREE_PARAMETER: f64 = 100.0;
