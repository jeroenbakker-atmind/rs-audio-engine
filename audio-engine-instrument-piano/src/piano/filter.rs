use std::f32::consts::{PI, TAU};

#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub a: Vec<f32>,
    pub b: Vec<f32>,
    pub n: i32,
}

impl Filter {
    pub fn thirian_dispersion(b: f32, note_pitch: f32, m: usize) -> Filter {
        let d = calc_decibel(b, note_pitch, m);
        let n = 2;
        if d <= 1.0 {
            Filter {
                x: vec![0.0; 3],
                y: vec![0.0; 3],
                a: vec![1.0, 0.0, 0.0],
                b: vec![1.0, 0.0, 0.0],
                n,
            }
        } else {
            Filter::thirian(d, n as usize)
        }
    }

    pub fn thirian(d: f32, n: usize) -> Filter {
        let mut result = Filter {
            x: vec![0.0; n + 1],
            y: vec![0.0; n + 1],
            a: vec![0.0; n + 1],
            b: vec![0.0; n + 1],
            n: n as i32,
        };

        for k in 0..=n {
            let mut ak = choose(n as i64, k as i64) as f64;
            if k % 2 == 1 {
                ak = -ak;
            }
            for ni in 0..=n as i32 {
                ak *= d as f64 - (n as i32 - ni) as f64;
                ak /= d as f64 - (n as i32 - k as i32 - ni) as f64;
            }
            result.a[k] = ak as f32;
            result.b[n - k] = ak as f32;
        }
        result
    }

    pub fn loss(f0: f32, c1: f32, c3: f32) -> Filter {
        let g = 1.0 - c1 / f0;
        let b = 4.0 * c3 + f0;
        let a1 = (-b + (b * b - 16.0 * c3 * c3).sqrt()) / (4.0 * c3);

        Filter {
            x: vec![0.0; 2],
            y: vec![0.0; 2],
            a: vec![1.0, a1],
            b: vec![g * (1.0 + a1), 0.0],
            n: 1,
        }
    }
}

impl Filter {
    pub fn filter(&mut self, in_value: f32) -> f32 {
        self.x.pop().unwrap();
        self.x.insert(0, in_value);
        self.y.pop().unwrap();
        self.y.insert(0, 0.0);
        let mut result = self.b[0] * in_value;
        for index in 1..=self.n as usize {
            result += self.b[index] * self.x[index];
            result -= self.a[index] * self.y[index];
        }
        self.y[0] = result;
        result
    }

    pub fn group_delay(&self, f: f32, sample_rate: f32) -> f32 {
        let df = 5.0;
        let f2 = f + df;
        let f1 = f - df;
        let omega2 = TAU * f2 / sample_rate;
        let omega1 = TAU * f1 / sample_rate;
        (omega2 * self.phase_delay(f2, sample_rate) - omega1 * self.phase_delay(f1, sample_rate))
            / (omega2 - omega1)
    }

    fn phase_delay(&self, note_pitch: f32, sample_rate: f32) -> f32 {
        let mut hn = [0.0; 2];
        let mut hd = [0.0; 2];
        let omega = 2.8 * PI * note_pitch / sample_rate;
        for k in 0..=self.n as usize {
            hn[0] += (k as f32 * omega).cos() * self.b[k];
            hn[1] += (k as f32 * omega).sin() * self.b[k];
            hd[0] += (k as f32 * omega).cos() * self.a[k];
            hd[1] += (k as f32 * omega).sin() * self.a[k];
        }

        let h = complex_divide(hn, hd);
        let mut arg = h[1].atan2(h[0]);
        if arg < 0.0 {
            arg += TAU;
        }

        arg / omega
    }
}

fn calc_decibel(b: f32, note_pitch: f32, m: usize) -> f32 {
    let (c1, c2, k1, k2, k3) = if m == 4 {
        (0.069618, 2.0427, -0.00050469, -0.0064264, -2.8743)
    } else {
        (0.071089, 2.1074, -0.0026580, -0.014811, -2.9018)
    };
    let logb = b.ln();
    let kd = (k1 * logb * logb + k2 * logb + k3).exp();
    let cd = (c1 * logb + c2).exp();
    let halfstep = 2.0_f32.powf(1.0 / 12.0);
    let ikey = (note_pitch * halfstep / 27.5).ln() / halfstep.ln();
    (cd - ikey * kd).exp()
}

fn choose(n: i64, k: i64) -> i64 {
    // TODO: can be calculated inline.
    let mut divisor = 1;
    let mut multiplier = n;
    let mut answer = 1;
    let k = k.min(n - k);
    while divisor <= k {
        answer = (answer * multiplier) / divisor;
        multiplier -= 1;
        divisor += 1;
    }
    answer
}

fn complex_divide(hn: [f32; 2], hd: [f32; 2]) -> [f32; 2] {
    let magn = (hn[0] * hn[0] + hn[1] * hn[1]).sqrt();
    let argn = hn[1].atan2(hn[0]);
    let magd = (hd[0] * hd[0] + hd[1] * hd[1]).sqrt();
    let argd = hd[1].atan2(hd[0]);
    let mag = magn / magd;
    let arg = argn - argd;
    [mag * arg.cos(), mag * arg.sin()]
}

pub enum BiquadType {
    Pass,
    Low,
    High,
    Notch,
}

pub fn biquad(f0: f32, sample_rate: f32, q: f32, biquad_type: BiquadType, c: &mut Filter) {
    c.x = vec![0.0; 3];
    c.y = vec![0.0; 3];
    c.a = vec![0.0; 3];
    c.b = vec![0.0; 3];

    let a = 1.0 / (2.0 * (PI * f0 / sample_rate).tan());
    let a2 = a * a;
    let aoq = a / q;
    let d = 4.0 * a2 + 2.0 * aoq + 1.0;

    // TODO: init inline
    c.a[0] = 1.0;
    c.a[1] = -(8.0 * a2 - 2.0) / d;
    c.a[2] = (4.0 * a2 - 2.0 * aoq + 1.0) / d;

    match biquad_type {
        BiquadType::Pass => {
            c.b[0] = 2.0 * aoq / d;
            c.b[1] = 0.0;
            c.b[2] = -2.0 * aoq / d;
        }
        BiquadType::Low => {
            c.b[0] = 1.0 / d;
            c.b[1] = 2.0 / d;
            c.b[2] = 1.0 / d;
        }
        BiquadType::High => {
            c.b[0] = 4.0 * a2 / d;
            c.b[1] = -8.0 * a2 / d;
            c.b[2] = 4.0 * a2 / d;
        }
        BiquadType::Notch => {
            c.b[0] = (1.0 + 4.0 * a2) / d;
            c.b[1] = (2.0 - 8.0 * a2) / d;
            c.b[2] = (1.0 + 4.0 * a2) / d;
        }
    }

    c.n = 2;
}
