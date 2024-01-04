#[derive(Debug, Default, Copy, Clone)]
pub struct Hammer {
    pub dt: f32,
    pub dti: f32,
    pub x: f32,
    pub v: f32,
    pub a: f32,
    pub s: i32,
    pub v0: f32,
    pub mi: f32,
    pub k: f32,
    pub p: f32,
    pub fs: f32,
    pub f: f32,
    pub upprev: f32,
    pub alpha: f32,
    pub z2i: f32,
}

impl Hammer {
    pub fn init(&mut self, f: f32, fs: f32, m: f32, k: f32, p: f32, z: f32, alpha: f32, v0: f32) {
        self.s = 3;
        self.fs = fs;
        self.p = p;
        self.k = k;
        self.mi = 1.0 / m;
        self.alpha = alpha;
        self.z2i = 1.0 / (z * 2.0);
        self.v0 = v0;
        self.dt = 1.0 / (fs * self.s as f32);
        // dti = fs * self.s
        self.dti = 1.0 / self.dt;
        self.x = 0.0;
        self.v = v0;
        self.a = 0.0;
        self.f = 0.0;
        self.upprev = 0.0;
    }

    pub fn load(&mut self, t: f32, vin: f32) -> f32 {
        for _ in 0..self.s {
            let up = if self.x > 0.0 {
                self.x.powf(self.p)
            } else {
                0.0
            };
            let mut dupdt = (up - self.upprev) * self.dti;
            let mut v1 = 0.0;
            let mut x1 = 0.0;
            for _ in 0..3 {
                self.f = (self.k * (up + self.alpha * dupdt)).max(0.0);
                self.a = -self.f * self.mi;
                v1 = self.v + self.a * self.dt;
                x1 = self.x + (v1 - (vin + self.f * self.z2i)) * self.dt;
                let upnew = if self.x > 0.0 {
                    self.x.powf(self.p)
                } else {
                    0.0
                };
                let dupdtnew = (upnew - self.upprev) / (2.0 * self.dt);
                let change = dupdtnew - dupdt;
                dupdt += 0.5 * change;
            }
            self.upprev = up;
            self.v = v1;
            self.x = x1;
        }
        self.f
    }
}
