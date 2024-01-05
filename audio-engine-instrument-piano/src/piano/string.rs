use std::{borrow::BorrowMut, cell::RefCell, rc::Rc};

use crate::piano::filter::{groupdelay, loss, thirian, thiriandispersion};

use super::{
    delay::{delay, init_delay, Delay},
    filter::{filter, Filter},
};

#[derive(Debug, Default, Clone)]
pub struct PianoString {
    pub m: usize,
    pub dispersion: Vec<Filter>,
    pub lowpass: Filter,
    pub fracdelay: Filter,
    pub d: [Dwg; 4],
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
            thiriandispersion(b, f, self.m, &mut self.dispersion[0]);
        } else {
            self.m = 4;
            for m in 0..self.m {
                thiriandispersion(b, f, self.m, &mut self.dispersion[m]);
            }
        }
        let dispersiondelay = self.m as f32 * groupdelay(&self.dispersion[0], f, fs);
        loss(f, fs, c1, c3, &mut self.lowpass);
        let lowpassdelay = groupdelay(&self.lowpass, f, fs);

        let del2 = ((0.5 * (deltot - 2.0 - del1 as f32) - dispersiondelay) as i32).max(1);
        let del3 = ((0.5 * (deltot - 2.0 * del1 as f32) - lowpassdelay - 5.0) as i32).max(1);

        let d = deltot - ((del1 + del1 + del2 + del3) as f32 + dispersiondelay + lowpassdelay);
        thirian(d, d.min(2.0) as usize, &mut self.fracdelay);
        // TODO: Unused
        let tuningdelay = groupdelay(&self.fracdelay, f, fs);

        self.d[0] = Dwg::new(z, del1, del1, 0);
        self.d[0] = Dwg::new(z, del2, del3, 0);
        self.d[0] = Dwg::new(zb, 0, 0, 0);
        self.d[0] = Dwg::new(zh, 0, 0, 0);
        self.d[0].connect_right(DwgNodeRef::Dwg1Left);
        self.d[1].connect_left(DwgNodeRef::Dwg0Right);
        self.d[1].connect_right(DwgNodeRef::Dwg2Left);
        self.d[2].connect_left(DwgNodeRef::Dwg1Right);
        self.d[0].connect_right(DwgNodeRef::Dwg3Left);
        self.d[1].connect_left(DwgNodeRef::Dwg3Left);
        self.d[3].connect_left(DwgNodeRef::Dwg0Right);
        self.d[3].connect_left(DwgNodeRef::Dwg1Left);
        let parent = self.clone();
        self.d[0].init(&parent);
        self.d[1].init(&parent);
        self.d[2].init(&parent);
        self.d[3].init(&parent);
    }

    fn get_node(&self, dwg_node_ref: DwgNodeRef) -> &'_ DwgNode {
        match dwg_node_ref {
            DwgNodeRef::Dwg0Left => &self.d[0].l,
            DwgNodeRef::Dwg0Right => &self.d[0].r,
            DwgNodeRef::Dwg1Left => &self.d[1].l,
            DwgNodeRef::Dwg1Right => &self.d[1].r,
            DwgNodeRef::Dwg2Left => &self.d[2].l,
            DwgNodeRef::Dwg2Right => &self.d[2].r,
            DwgNodeRef::Dwg3Left => &self.d[3].l,
            DwgNodeRef::Dwg3Right => &self.d[3].r,
            DwgNodeRef::Null => panic!(),
        }
    }

    pub fn input_velocity(&self) -> f32 {
        self.d[1].l.a[0] + self.d[0].r.a[1]
    }

    pub fn go_hammer(&mut self, load: f32) -> f32 {
        self.d[3].l.load = load;
        for k in 0..2 {
            self.d[k].dodelay();
        }
        self.d[1].r.a[1]
    }

    pub fn go_soundboard(&mut self, load: f32) -> f32 {
        self.d[2].l.load = load;
        for k in 0..3 {
            let parent = self.clone();
            self.d[k].doload(&parent);
        }

        for k in 0..3 {
            self.update(k);
        }

        self.d[2].l.a[1]
    }

    pub fn update(&mut self, k: usize) {
        let dwg = &mut self.d[k];
        let mut a = dwg.loadl - dwg.l.a[0];
        if dwg.commute != 0 {
            for m in 0..self.m {
                a = filter(a, &mut self.dispersion[m]);
            }
        }
        dwg.l.a[1] = a;

        let mut a = dwg.loadr - dwg.r.a[1];
        if dwg.commute != 0 {
            a = filter(a, &mut self.lowpass);
            a = filter(a, &mut self.fracdelay);
        }
        dwg.r.a[0] = a;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Dwg {
    pub del1: i32,
    pub del2: i32,

    // TODO: Move into node+polarity struct
    // TODO: Remove nl and nr.
    pub nl: usize,
    pub nr: usize,
    pub pl: [i32; 2],
    pub pr: [i32; 2],
    pub cl: [DwgNodeRef; 2],
    pub cr: [DwgNodeRef; 2],
    pub l: DwgNode,
    pub r: DwgNode,
    pub loadl: f32,
    pub loadr: f32,
    pub al: [f32; 2],
    pub ar: [f32; 2],
    pub alphalthis: f32,
    pub alpharthis: f32,
    pub alphal: [f32; 2],
    pub alphar: [f32; 2],
    // TODO: make Option<Delay>
    pub d: [Delay; 2],
    // Is boolean
    pub commute: i32,
}

impl Dwg {
    pub fn new(z: f32, del1: i32, del2: i32, commute: i32) -> Dwg {
        let mut result = Dwg::default();

        if del1 > 1 {
            init_delay(&mut result.d[0], del1 - 1);
        }
        if del2 > 1 {
            init_delay(&mut result.d[1], del2 - 1);
        }

        result.del1 = del1;
        result.del2 = del2;
        result.l = DwgNode::new(z);
        result.r = DwgNode::new(z);
        result.commute = commute;

        result
    }

    fn connect_left_with_polarity(&mut self, l: DwgNodeRef, polarity: i32) {
        self.cl[self.nl] = l;
        self.pl[self.nl] = polarity;
        self.nl += 1
    }

    fn connect_right_with_polarity(&mut self, r: DwgNodeRef, polarity: i32) {
        self.cr[self.nr] = r;
        self.pr[self.nr] = polarity;
        self.nr += 1
    }

    pub fn connect_left(&mut self, l: DwgNodeRef) {
        self.connect_left_with_polarity(l, 0);
    }
    pub fn connect_right(&mut self, r: DwgNodeRef) {
        self.connect_right_with_polarity(r, 0);
    }

    pub fn init(&mut self, parent: &PianoString) {
        let mut ztot = self.l.z;
        for k in 0..self.nl {
            ztot += parent.get_node(self.cl[k]).z;
        }
        self.alphalthis = 2.0 * self.l.z / ztot;
        for k in 0..self.nl {
            self.alphal[k] = 2.0 * parent.get_node(self.cl[k]).z / ztot;
        }

        let mut ztot = self.r.z;
        for k in 0..self.nr {
            ztot += parent.get_node(self.cr[k]).z;
        }
        self.alpharthis = 2.0 * self.r.z / ztot;
        for k in 0..self.nr {
            self.alphar[k] = 2.0 * parent.get_node(self.cr[k]).z / ztot;
        }
    }

    pub fn dodelay(&mut self) {
        let dar = if self.del1 < 2 {
            self.r.a[0]
        } else {
            delay(self.r.a[0], &mut self.d[0])
        };

        let dal = if self.del2 < 2 {
            self.l.a[0]
        } else {
            delay(self.l.a[0], &mut self.d[1])
        };
        self.l.a[0] = dar;
        self.r.a[1] = dal;
    }
    pub fn doload(&mut self, parent: &PianoString) {
        if self.nl == 0 {
            self.loadl = 0.0;
        } else {
            self.loadl = self.alphalthis * self.l.a[0];
            for k in 0..self.nl {
                let polarity = if self.pl[k] != 0 { 0 } else { 1 };
                let node = parent.get_node(self.cl[k]);
                self.loadl += node.load;
                self.loadl += self.alphal[k] * node.a[polarity];
            }
        }

        if self.nr == 0 {
            self.loadr = 0.0;
        } else {
            self.loadr = self.alpharthis * self.r.a[1];
            for k in 0..self.nr {
                let polarity = if self.pr[k] != 0 { 1 } else { 0 };
                let node = parent.get_node(self.cr[k]);
                self.loadr += node.load;
                self.loadr += self.alphar[k] * node.a[polarity];
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DwgNode {
    pub z: f32,
    pub load: f32,
    pub a: [f32; 2],
}

impl DwgNode {
    pub fn new(z: f32) -> DwgNode {
        DwgNode {
            z,
            ..DwgNode::default()
        }
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub enum DwgNodeRef {
    #[default]
    Null,
    Dwg0Left,
    Dwg0Right,
    Dwg1Left,
    Dwg1Right,
    Dwg2Left,
    Dwg2Right,
    Dwg3Left,
    Dwg3Right,
}
