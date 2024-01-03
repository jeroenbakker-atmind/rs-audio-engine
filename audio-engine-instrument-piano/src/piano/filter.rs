use std::f32::consts::PI;

pub struct Filter {
    pub x: Vec<f32>,
    pub y: Vec<f32>,
    pub a: Vec<f32>,
    pub b: Vec<f32>,
    pub n: i32,
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
