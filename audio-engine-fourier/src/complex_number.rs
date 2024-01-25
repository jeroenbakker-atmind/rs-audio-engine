/// ComplexNumber can hold the real and imaginary part of a complex number.
///
/// Doing audio processing it isn't really necessary to know how complex
/// number work. See the `real` part as being associated with cosine and
/// the imaginary part with sine. The terms complex number, real and
/// imaginary are kept for alignment with other materials about fourier
/// transforms.
pub type ComplexNumber = (f32, f32);
pub trait ComplexNumberMethods {
    fn amplitude(&self) -> f32;
}
impl ComplexNumberMethods for ComplexNumber {
    fn amplitude(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
}
