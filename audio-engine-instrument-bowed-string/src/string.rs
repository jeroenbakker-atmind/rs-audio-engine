#[derive(Debug, Default, Copy, Clone)]
pub struct String {
    /// Radius of the string in meters.
    pub radius: f32,
    /// Density of the string in kg/m.
    pub density: f32,
    /// Tension of the string in newtons.
    pub tension: f32,
    /// Young modulus of the string in Pa.
    pub young_mod: f32,
    /// Length of the string in meters.
    pub length: f32,
}
