use std::f32::consts::PI;

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

impl String {
    pub fn area(&self) -> f32 {
        PI * self.radius * self.radius
    }

    pub fn lin_density(&self) -> f32 {
        self.density * self.area()
    }

    pub fn inertia(&self) -> f32 {
        PI * self.radius * self.radius * self.radius * self.radius / 4.0
    }

    pub fn c(&self) -> f32 {
        self.tension / self.lin_density()
    }
}
