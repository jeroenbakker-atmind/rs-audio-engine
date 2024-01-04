use crate::piano::filter::tririandispersion;

use super::filter::Filter;

#[derive(Default, Clone)]
pub struct PianoString {
    pub m: usize,
    pub dispersion: Vec<Filter>,
}

impl PianoString {
    pub fn init(
        &mut self,
        f: f32,
        fs: f32,
        inpos: f32,
        c1: f32,
        c3: f32,
        b: f32,
        z: f32,
        zb: f32,
        zh: f32,
    ) {
        let deltot = fs / f;
        let del1 = ((inpos * 0.5 * deltot) as i32).max(1);

        self.dispersion.clear();
        if f > 400.0 {
            self.m = 1;
            self.dispersion.resize(1, Filter::default());
            tririandispersion(b, f, self.m, &mut self.dispersion[0]);
        } else {
            self.m = 4;
            for m in 0..self.m {
                tririandispersion(b, f, self.m, &mut self.dispersion[m]);
            }
        }
        unimplemented!()
    }
}
