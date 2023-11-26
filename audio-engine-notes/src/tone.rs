use crate::Scale;

pub trait Tone {
    type ScaleType: Scale;

    fn scale() -> Self::ScaleType;
}
