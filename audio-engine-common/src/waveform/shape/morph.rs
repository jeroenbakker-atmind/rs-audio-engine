use super::{
    saw::SawShape, sine::SineShape, square::SquareShape, triangle::TriangleShape, HarmonicShape,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct MorphShape {
    pub x: f32,
    pub y: f32,
    sine: SineShape,
    square: SquareShape,
    triangle: TriangleShape,
    saw: SawShape,
}

impl MorphShape {
    pub fn new(x: f32, y: f32) -> MorphShape {
        MorphShape {
            x,
            y,
            ..MorphShape::default()
        }
    }
}

impl HarmonicShape for MorphShape {
    fn get_harmonic_amplitude(&self, harmonic: u8) -> f32 {
        let x = self.x.clamp(0.0, 1.0);
        let y = self.y.clamp(0.0, 1.0);
        let inv_x = 1.0 - x;
        let inv_y = 1.0 - y;

        let sine_area = inv_x * inv_y;
        let square_area = inv_x * y;
        let triangle_area = x * inv_y;
        let saw_area = x * y;

        sine_area * self.sine.get_harmonic_amplitude(harmonic)
            + square_area * self.square.get_harmonic_amplitude(harmonic)
            + triangle_area * self.triangle.get_harmonic_amplitude(harmonic)
            + saw_area * self.saw.get_harmonic_amplitude(harmonic)
    }
}
