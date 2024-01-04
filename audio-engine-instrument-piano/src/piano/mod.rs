pub mod delay;
pub mod filter;
pub mod hammer;
pub mod reverb;
pub mod string;

use std::f32::consts::PI;

use self::{
    filter::{biquad, BiquadType, Filter},
    hammer::Hammer,
    reverb::Reverb,
    string::PianoString,
};

pub struct Piano {
    pub v0: f32,
    pub samples: usize,
    pub sample: usize,
    pub fs: f32,
    pub t: f32,
    pub dt: f32,
    pub z: f32,
    pub zb: f32,
    pub zh: f32,
    pub f: f32,

    pub strings: Vec<PianoString>,
    pub hammer: Hammer,
    pub soundboard: Reverb,
    pub shaping1: Filter,
    pub shaping2: Filter,
    pub shaping3: Filter,
}

impl Piano {
    // TODO: Piano::new()->Piano
    pub fn init(&mut self, note_pitch: f32, fs: f32, v0: f32, samples: usize) {
        self.fs = fs;
        self.v0 = v0;
        self.samples = samples;
        self.sample = 0;
        self.t = 0.0;
        self.dt = 1.0 / fs;
        self.f = note_pitch;

        let f0 = 27.5;
        let rho = 7850.0;
        // TODO: extract ln/ln into local variable.
        let p = 2.0 + 1.0 * (self.f / f0).ln() / (4192.0 / f0).ln();
        let m = 0.06 - 0.058 * ((self.f / f0).ln() / (4192.0 / f0).ln()).powf(0.1);
        let k = 40.0 / (0.7e-3_f32).powf(p);
        // TODO: is unused?
        let l = 1.4 - 1.0 / 32.0 * (self.f / f0).ln() / (4192.0 / f0).ln();
        let l = 0.04 + 1.4 / (1.0 + (-3.4 + 1.4 * (self.f / f0).ln())).exp();
        let r = 0.002 * (1.0 + 0.6 * (self.f / f0).ln().powf(-1.4));
        let rhol = PI * r * r * rho;
        // TODO: Use sqr?
        let t = (2.0 * l * self.f) * (2.0 * l * self.f) * rhol;
        self.z = (t * rhol).sqrt();
        self.zb = 4000.0;
        self.zh = 0.0;

        let e = 200e9;
        // TODO: unused?
        let flong = (e / rho).sqrt() / (2.0 * l);
        // TODO: use min
        let rcore = if (r < 0.0006) { r } else { 0.0006 };
        let b = (PI * PI * PI) * e * (rcore * rcore * rcore * rcore) / (4.0 * l * l * t);
        let hp = 1.0 / 7.0;

        let c1 = 0.25;
        let c3 = 5.85;
        let c1b = 20.0;
        let c3b = 20.0;

        // 24.5 = g0 == midinote 31
        // 43.65 = f1 = midinote 41
        let num_strings = if note_pitch < 24.5 {
            1
        } else if note_pitch < 43.65 {
            2
        } else {
            3
        };
        self.strings.clear();
        self.strings.resize(num_strings, PianoString::default());

        let string_detunings = [1.0, 1.0003, 0.9996];
        for string_index in 0..num_strings {
            let detune = string_detunings[string_index];
            // TODO: self.strings.push(PianoString::new());
            self.strings[string_index].init(
                self.f * detune,
                fs,
                hp,
                c1,
                c3,
                b,
                self.z,
                self.zb + (string_index as f32 - 1.0) * self.z,
                self.zh,
            );
        }

        let a = -1.0 / 4.0;
        let mix = 1.0;
        let alpha = 0.1e-4_f32 * (self.f / f0).ln() / (4192.0 / f0).ln();
        // TODO: self.soundboard = Reverb::new()
        self.soundboard.init(c1b, c3b, a, mix, fs);
        // TODO: self.hammer = Hammer::new()
        self.hammer.init(self.f, fs, m, k, p, self.z, alpha, v0);

        // TODO: self.shaping1 = Filter::biquad()
        biquad(500.0, fs, 10.0, BiquadType::Notch, &mut self.shaping1);
        biquad(200.0, fs, 1.0, BiquadType::High, &mut self.shaping2);
        biquad(800.0, fs, 1.0, BiquadType::Low, &mut self.shaping3);
    }
}
