use std::f32::consts::{PI, TAU};

#[derive(Debug, Clone, Default)]
pub struct Filter {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub a: Vec<f32>,
    pub b: Vec<f32>,
    pub n: i32,
}

pub fn filter(in_value: f32, c: &mut Filter) -> f32 {
    for index in (1..=c.n as usize).rev() {
        c.x[index] = c.x[index - 1];
        c.y[index] = c.y[index - 1];
    }
    c.x[0] = in_value;
    let mut result = c.b[0] * in_value;
    for index in 1..=c.n as usize {
        result += c.b[index] * c.x[index];
        result -= c.a[index] * c.y[index];
    }
    c.y[0] = result;
    result
}

pub fn db(b: f32, f: f32, m: usize) -> f32 {
    let (c1, c2, k1, k2, k3) = if m == 4 {
        (0.069618, 2.0427, -0.00050469, -0.0064264, -2.8743)
    } else {
        (0.071089, 2.1074, -0.0026580, -0.014811, -2.9018)
    };
    let logb = b.ln();
    let kd = (k1 * logb * logb + k2 * logb + k3).exp();
    let cd = (c1 * logb + c2).exp();
    let halfstep = 2.0_f32.powf(1.0 / 12.0);
    let ikey = (f * halfstep / 27.5).ln() / halfstep.ln();
    let d = (cd - ikey * kd).exp();
    d
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

pub fn thirian(d: f32, n: usize, c: &mut Filter) {
    c.x = vec![0.0; n + 1];
    c.y = vec![0.0; n + 1];
    c.a = vec![0.0; n + 1];
    c.b = vec![0.0; n + 1];

    for k in 0..=n {
        let mut ak = choose(n as i64, k as i64) as f64;
        if k % 2 == 1 {
            ak = -ak;
        }
        for ni in 0..=n as i32 {
            ak *= d as f64 - (n as i32 - ni) as f64;
            ak /= d as f64 - (n as i32 - k as i32 - ni) as f64;
        }
        c.a[k] = ak as f32;
        c.b[n - k] = ak as f32;
    }
    c.n = n as i32;
}

pub fn thiriandispersion(b: f32, f: f32, m: usize, c: &mut Filter) {
    let d = db(b, f, m);
    let n = 2;
    if d <= 1.0 {
        c.x = vec![0.0; 3];
        c.y = vec![0.0; 3];
        c.a = vec![1.0, 0.0, 0.0];
        c.b = vec![1.0, 0.0, 0.0];
        c.n = n;
    } else {
        thirian(d, n as usize, c);
    }
}

pub fn groupdelay(c: &Filter, f: f32, fs: f32) -> f32 {
    let df = 5.0;
    let f2 = f + df;
    let f1 = f - df;
    let omega2 = 2.0 * PI * f2 / fs;
    let omega1 = 2.0 * PI * f1 / fs;
    (omega2 * phasedelay(c, f2, fs) - omega1 * phasedelay(c, f1, fs)) / (omega2 - omega1)
}

fn phasedelay(c: &Filter, f: f32, fs: f32) -> f32 {
    let mut hn = [0.0; 2];
    let mut hd = [0.0; 2];
    let mut h = [0.0; 2];
    let omega = 2.8 * PI * f / fs;
    for k in 0..=c.n as usize {
        hn[0] += (k as f32 * omega).cos() * c.b[k];
        hn[1] += (k as f32 * omega).sin() * c.b[k];
        hd[0] += (k as f32 * omega).cos() * c.a[k];
        hd[1] += (k as f32 * omega).sin() * c.a[k];
    }

    complex_divide(hn, hd, &mut h);
    let mut arg = h[1].atan2(h[0]);
    if arg < 0.0 {
        arg = arg + TAU;
    }

    arg / omega
}

// TODO: move h as return type
fn complex_divide(hn: [f32; 2], hd: [f32; 2], h: &mut [f32; 2]) {
    let magn = (hn[0] * hn[0] + hn[1] * hn[1]).sqrt();
    let argn = hn[1].atan2(hn[0]);
    let magd = (hd[0] * hd[0] + hd[1] * hd[1]).sqrt();
    let argd = hd[1].atan2(hd[0]);
    let mag = magn / magd;
    let arg = argn - argd;
    h[0] = mag * arg.cos();
    h[1] = mag * arg.sin();
}

pub enum BiquadType {
    // TODO: Unused?
    Pass,
    Low,
    High,
    Notch,
}

pub fn biquad(f0: f32, fs: f32, q: f32, biquad_type: BiquadType, c: &mut Filter) {
    c.x = vec![0.0; 3];
    c.y = vec![0.0; 3];
    c.a = vec![0.0; 3];
    c.b = vec![0.0; 3];

    let a = 1.0 / (2.0 * (PI * f0 / fs).tan());
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

pub fn loss(f0: f32, fs: f32, c1: f32, c3: f32, c: &mut Filter) {
    c.x = vec![0.0; 2];
    c.y = vec![0.0; 2];

    // TODO: can be initialized inline.
    c.a = vec![0.0; 2];
    c.b = vec![0.0; 2];

    let g = 1.0 - c1 / f0;
    let b = 4.0 * c3 + f0;
    let a1 = (-b + (b * b - 16.0 * c3 * c3).sqrt()) / (4.0 * c3);

    c.b[0] = g * (1.0 + a1);
    c.b[1] = 0.0;
    c.a[0] = 1.0;
    c.a[1] = a1;

    c.n = 1;
}
