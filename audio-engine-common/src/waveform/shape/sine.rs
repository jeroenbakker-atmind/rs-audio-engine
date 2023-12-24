use super::Shape;

pub struct SineShape {}
impl Shape for SineShape {
    fn get_shape_amplitude(&self, harmonic: u8) -> f32 {
        if harmonic == 1 {
            1.0
        } else {
            0.0
        }
    }
}
