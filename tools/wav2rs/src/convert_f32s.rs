use wav::BitDepth;

pub trait Convert2F32 {
    fn to_f32s(&self) -> Vec<f32>;
}

impl Convert2F32 for BitDepth {
    fn to_f32s(&self) -> Vec<f32> {
        let mut result = Vec::new();
        match self {
            BitDepth::Eight(data) => {
                for elem in data {
                    let sample = *elem as f32 / 127.0;
                    result.push(sample);
                }
            }
            BitDepth::Sixteen(data) => {
                for elem in data {
                    let sample = *elem as f32 / 32768.0;
                    result.push(sample);
                }
            }
            BitDepth::TwentyFour(data) => {
                for elem in data {
                    let sample = *elem as f32 / 8388607.0;
                    result.push(sample);
                }
            }

            BitDepth::ThirtyTwoFloat(data) => {
                for elem in data {
                    result.push(*elem)
                }
            }

            _ => unimplemented!("unsupported bit depth."),
        }
        result
    }
}
