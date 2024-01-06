pub mod delay;
pub mod filter;
pub mod hammer;
pub mod reverb;
pub mod string;
#[cfg(test)]
mod test;

use std::f32::consts::PI;

use self::{
    filter::{biquad, BiquadType, Filter},
    hammer::Hammer,
    reverb::Reverb,
    string::PianoString,
};

#[derive(Debug, Default, Clone)]
pub struct Piano {
    // Hammer Velocity (between 0..10) in m/s
    pub hammer_velocity: f32,
    // Sample rate
    pub sample_rate: f32,
    pub delta_time: f32,
    pub z: f32,
    pub zb: f32,
    pub zh: f32,
    pub note_pitch: f32,

    pub strings: Vec<PianoString>,
    pub hammer: Hammer,
    pub soundboard: Reverb,
    pub shaping1: Filter,
    pub shaping2: Filter,
    pub shaping3: Filter,
}

impl Piano {
    // TODO: Piano::new()->Piano
    pub fn init(&mut self, note_pitch: f32, sample_rate: f32, hammer_velocity: f32) {
        self.sample_rate = sample_rate;
        self.hammer_velocity = hammer_velocity;
        self.delta_time = 1.0 / sample_rate;
        self.note_pitch = note_pitch;

        let f0 = 27.5;
        let rho = 7850.0;
        // TODO: extract ln/ln into local variable.
        let p = 2.0 + 1.0 * (self.note_pitch / f0).ln() / (4192.0 / f0).ln();
        let m = 0.06 - 0.058 * ((self.note_pitch / f0).ln() / (4192.0 / f0).ln()).powf(0.1);
        let k = 40.0 / (0.7e-3_f32).powf(p);
        let l = 0.04 + 1.4 / (1.0 + (-3.4 + 1.4 * (self.note_pitch / f0).ln()).exp());
        let r = 0.002 * (1.0 + 0.6 * (self.note_pitch / f0).ln()).powf(-1.4);
        let rhol = PI * r * r * rho;
        let t = (2.0 * l * self.note_pitch).powi(2) * rhol;
        self.z = (t * rhol).sqrt();
        self.zb = 4000.0;
        self.zh = 0.0;

        let e = 200e9;
        let rcore = r.min(0.0006);
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

        for (string_index, detune) in [1.0, 1.0003, 0.9996].iter().take(num_strings).enumerate() {
            // TODO: self.strings.push(PianoString::new())
            self.strings[string_index].init(
                self.note_pitch * detune,
                sample_rate,
                hp,
                c1,
                c3,
                b,
                self.z,
                self.zb + (num_strings as f32 - 1.0) * self.z,
                self.zh,
            );
        }

        let a = -1.0 / 4.0;
        let mix = 1.0;
        let alpha = 0.1e-4_f32 * (self.note_pitch / f0).ln() / (4192.0 / f0).ln();
        self.soundboard = Reverb::new(c1b, c3b, a, mix, sample_rate);
        // TODO: self.hammer = Hammer::new()
        self.hammer
            .init(sample_rate, m, k, p, self.z, alpha, hammer_velocity);

        // TODO: self.shaping1 = Filter::biquad()
        biquad(
            500.0,
            sample_rate,
            10.0,
            BiquadType::Notch,
            &mut self.shaping1,
        );
        biquad(
            200.0,
            sample_rate,
            1.0,
            BiquadType::High,
            &mut self.shaping2,
        );
        biquad(800.0, sample_rate, 1.0, BiquadType::Low, &mut self.shaping3);
    }

    pub fn go(&mut self, samples_out: &mut [f32]) {
        let string_len = self.strings.len() as f32;
        for sample_out in samples_out {
            let mut vstring = 0.0;
            for string in &self.strings {
                vstring += string.input_velocity();
            }
            let hload = self.hammer.load(vstring / string_len);
            let mut load = 0.0;
            for string in &mut self.strings {
                load += (2.0 * self.z * string.do_hammer(hload / (2.0 * self.z)))
                    / (self.z * string_len + self.zb);
            }

            let mut output = 0.0;
            for string in &mut self.strings {
                output += string.do_soundboard(load);
            }

            output = self.soundboard.reverb(output);
            *sample_out = output * 100.0;
        }
    }
}
