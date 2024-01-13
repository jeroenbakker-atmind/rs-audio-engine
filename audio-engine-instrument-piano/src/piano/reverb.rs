use super::{delay::Delay, filter::Filter};

#[derive(Default, Debug, Clone)]
pub struct Reverb {
    pub mix: f32,
    pub delays: [Delay; 8],
    pub a: [[f32; 8]; 8],
    pub o: [f32; 8],
    pub b: [f32; 8],
    pub c: [f32; 8],
    pub decay: [Filter; 8],
}

impl Reverb {
    pub fn new(c1: f32, c3: f32, a: f32, mix: f32, sample_rate: f32) -> Reverb {
        let mut result = Reverb::default();
        result.init(c1, c3, a, mix, sample_rate);
        result
    }

    fn init(&mut self, c1: f32, c3: f32, a: f32, mix: f32, sample_rate: f32) {
        self.mix = mix;
        let lengths = [37, 87, 181, 271, 359, 592, 687, 721];
        let aa = [a, a + 1.0, a, a, a, a, a, a];

        for (k, length) in lengths.iter().enumerate() {
            self.delays[k] = Delay::new(*length);
            self.o[k] = 0.0;
            self.b[k] = 1.0;
            self.c[k] = if k % 2 == 0 { 1.0 / 8.0 } else { -1.0 / 8.0 };

            self.decay[k] = Filter::loss(sample_rate / *length as f32, c1, c3);
        }

        for j in 0..8 {
            for k in 0..8 {
                let index = (8 + (k as i32 - j as i32)) % 8;
                self.a[j][k] = aa[index as usize];
            }
        }
    }

    pub fn reverb(&mut self, in_value: f32) -> f32 {
        let mut i = [0.0_f32; 8];
        for j in 0..8 {
            i[j] = self.b[j] * in_value;
            for k in 0..8 {
                i[j] += self.a[j][k] * self.o[k];
            }
        }

        let mut result = 0.0;
        for j in 0..8 {
            self.o[j] = self.decay[j].filter(self.delays[j].delay(i[j]));
            result += self.c[j] * self.o[j] * 0.5;
        }

        self.mix * result + (1.0 - self.mix) * in_value
    }
}
