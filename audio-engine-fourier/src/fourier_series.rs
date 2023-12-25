use std::f32::consts::TAU;

use crate::parameters::Parameters;

pub struct FourierSeries {
    /// Parameters of this series.
    pub parameters: Parameters,

    pub amplitudes: Vec<f32>,
}

impl FourierSeries {
    pub fn collect_radian_speed_with_amplitude(&self) -> Vec<(f32, f32)> {
        let mut integer_step = 0;
        let mut sub_step = 0;
        self.amplitudes
            .iter()
            .map(|amplitude| {
                let radian_speed = TAU
                    * (integer_step as f32
                        + sub_step as f32 / self.parameters.sub_step.max(1) as f32);
                sub_step += 1;
                if sub_step > self.parameters.sub_step {
                    integer_step += 1;
                    sub_step = 0;
                }
                (radian_speed, *amplitude)
            })
            .collect::<Vec<(f32, f32)>>()
    }
}
