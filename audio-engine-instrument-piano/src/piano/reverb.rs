use super::{
    delay::{delay, init_delay, Delay},
    filter::{filter, loss, Filter},
};

pub struct Reverb {
    pub mix: f32,
    pub d: [Delay; 8],
    pub a: [[f32; 8]; 8],
    pub o: [f32; 8],
    pub b: [f32; 8],
    pub c: [f32; 8],
    pub decay: [Filter; 8],
}

impl Reverb {
    pub fn init(&mut self, c1: f32, c3: f32, a: f32, mix: f32, fs: f32) {
        self.mix = mix;
        let lengths = [37, 87, 181, 271, 359, 592, 687, 721];
        let aa = [a, a + 1.0, a, a, a, a, a, a];

        for k in 0..8 {
            init_delay(&mut self.d[k], lengths[k]);
            self.o[k] = 0.0;
            self.b[k] = 1.0;
            // TODO: k is always smaller than 8?
            self.c[k] = if k < 8 {
                if k % 2 == 0 {
                    1.0 / 8.0
                } else {
                    -1.0 / 8.0
                }
            } else {
                0.0
            };

            loss(fs / lengths[k] as f32, fs, c1, c3, &mut self.decay[k]);
        }

        for j in 0..8 {
            for k in 0..8 {
                self.a[j][k] = aa[(k + (k - j)) % 8];
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
            self.o[j] = filter(delay(i[j], &mut self.d[j]), &mut self.decay[j]);
            result += self.c[j] * self.o[j] * 0.5;
        }

        self.mix * result + (1.0 - self.mix) * in_value
    }
}
