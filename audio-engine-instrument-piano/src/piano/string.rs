use super::{delay::Delay, filter::Filter};

#[derive(Debug, Default, Clone)]
pub struct PianoString {
    /// Number of dispersion filters
    pub m: usize,
    pub dispersion: Vec<Filter>,
    pub low_pass: Filter,
    pub fracdelay: Filter,
    pub d: [Dwg; 4],
}

impl PianoString {
    pub fn init(
        &mut self,
        note_pitch: f32,
        sample_rate: f32,
        inpos: f32,
        c1: f32,
        c3: f32,
        b: f32,
        z: f32,
        zb: f32,
        zh: f32,
    ) {
        let deltot = sample_rate / note_pitch;
        let del1 = ((inpos * 0.5 * deltot) as i32).max(1);

        self.dispersion.clear();
        if note_pitch > 400.0 {
            self.m = 1;
            self.dispersion.resize(1, Filter::default());
            self.dispersion[0] = Filter::thirian_dispersion(b, note_pitch, self.m);
        } else {
            self.m = 4;
            self.dispersion.resize(4, Filter::default());
            for m in 0..self.m {
                self.dispersion[m] = Filter::thirian_dispersion(b, note_pitch, self.m);
            }
        }
        let dispersion_delay =
            self.m as f32 * self.dispersion[0].group_delay(note_pitch, sample_rate);
        self.low_pass = Filter::loss(note_pitch, c1, c3);
        let low_pass_delay = self.low_pass.group_delay(note_pitch, sample_rate);

        let del2 = ((0.5 * (deltot - 2.0 * del1 as f32) - dispersion_delay) as i32).max(1);
        let del3 = ((0.5 * (deltot - 2.0 * del1 as f32) - low_pass_delay - 5.0) as i32).max(1);

        let d = deltot - ((del1 + del1 + del2 + del3) as f32 + dispersion_delay + low_pass_delay);
        self.fracdelay = Filter::thirian(d, d as usize);

        self.d[0] = Dwg::new(z, del1, del1, false);
        self.d[1] = Dwg::new(z, del2, del3, true);
        self.d[2] = Dwg::new(zb, 0, 0, false);
        self.d[3] = Dwg::new(zh, 0, 0, false);
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
        let parent = self.clone();
        self.d[1].init(&parent);
        let parent = self.clone();
        self.d[2].init(&parent);
        let parent = self.clone();
        self.d[3].init(&parent);
    }

    fn get_node(&self, dwg_node_ref: DwgNodeRef) -> &'_ DwgNode {
        match dwg_node_ref {
            DwgNodeRef::Dwg0Left => &self.d[0].left,
            DwgNodeRef::Dwg0Right => &self.d[0].right,
            DwgNodeRef::Dwg1Left => &self.d[1].left,
            DwgNodeRef::Dwg1Right => &self.d[1].right,
            DwgNodeRef::Dwg2Left => &self.d[2].left,
            DwgNodeRef::Dwg2Right => &self.d[2].right,
            DwgNodeRef::Dwg3Left => &self.d[3].left,
            DwgNodeRef::Dwg3Right => &self.d[3].right,
        }
    }

    pub fn input_velocity(&self) -> f32 {
        self.d[1].left.a[0] + self.d[0].right.a[1]
    }

    pub fn do_hammer(&mut self, load: f32) -> f32 {
        self.d[3].left.load = load;
        for k in 0..2 {
            self.d[k].do_delay();
        }
        self.d[1].right.a[1]
    }

    pub fn do_soundboard(&mut self, load: f32) -> f32 {
        self.d[2].left.load = load;
        for k in 0..3 {
            let parent = self.clone();
            self.d[k].do_load(&parent);
        }

        for k in 0..3 {
            self.update(k);
        }

        self.d[2].left.a[1]
    }

    pub fn update(&mut self, k: usize) {
        let dwg = &mut self.d[k];
        let mut a = dwg.load_left - dwg.left.a[0];
        if dwg.commute {
            for m in 0..self.m {
                a = self.dispersion[m].filter(a);
            }
        }
        dwg.left.a[1] = a;

        let mut a = dwg.load_right - dwg.right.a[1];
        if dwg.commute {
            a = self.low_pass.filter(a);
            a = self.fracdelay.filter(a);
        }
        dwg.right.a[0] = a;
    }
}

#[derive(Debug, Clone)]
pub struct DwgConnection {
    pub node_ref: DwgNodeRef,
    pub alpha: f32,
}
impl DwgConnection {
    fn new(node_ref: DwgNodeRef) -> DwgConnection {
        DwgConnection {
            node_ref,
            alpha: 0.0,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Dwg {
    pub left: DwgNode,
    pub right: DwgNode,
    pub delay_1: Option<Delay>,
    pub delay_2: Option<Delay>,
    pub commute: bool,

    pub left_connections: Vec<DwgConnection>,
    pub right_connections: Vec<DwgConnection>,

    pub load_left: f32,
    pub load_right: f32,
    pub alpha_left: f32,
    pub alpha_right: f32,
}

impl Dwg {
    pub fn new(z: f32, delay_1: i32, delay_2: i32, commute: bool) -> Dwg {
        Dwg {
            delay_1: if delay_1 > 1 {
                Some(Delay::new(delay_1 - 1))
            } else {
                None
            },
            delay_2: if delay_2 > 1 {
                Some(Delay::new(delay_2 - 1))
            } else {
                None
            },
            left: DwgNode::new(z),
            right: DwgNode::new(z),
            commute,
            ..Default::default()
        }
    }

    pub fn connect_left(&mut self, node_ref: DwgNodeRef) {
        self.left_connections.push(DwgConnection::new(node_ref));
    }

    pub fn connect_right(&mut self, node_ref: DwgNodeRef) {
        self.right_connections.push(DwgConnection::new(node_ref));
    }

    pub fn init(&mut self, parent: &PianoString) {
        let mut sum_z = self.left.z;
        for connection in &self.left_connections {
            sum_z += parent.get_node(connection.node_ref).z;
        }
        self.alpha_left = 2.0 * self.left.z / sum_z;
        for connection in self.left_connections.iter_mut() {
            connection.alpha = 2.0 * parent.get_node(connection.node_ref).z / sum_z;
        }

        let mut sum_z = self.right.z;
        for connection in &self.right_connections {
            sum_z += parent.get_node(connection.node_ref).z;
        }
        self.alpha_right = 2.0 * self.right.z / sum_z;
        for connection in self.right_connections.iter_mut() {
            connection.alpha = 2.0 * parent.get_node(connection.node_ref).z / sum_z;
        }
    }

    pub fn do_delay(&mut self) {
        let dar = if let Some(delay) = &mut self.delay_1 {
            delay.delay(self.right.a[0])
        } else {
            self.right.a[0]
        };

        let dal = if let Some(delay) = &mut self.delay_2 {
            delay.delay(self.left.a[1])
        } else {
            self.left.a[1]
        };

        self.left.a[0] = dar;
        self.right.a[1] = dal;
    }

    pub fn do_load(&mut self, parent: &PianoString) {
        if self.left_connections.is_empty() {
            self.load_left = 0.0;
        } else {
            self.load_left = self.alpha_left * self.left.a[0];
            for connection in self.left_connections.iter() {
                let node = parent.get_node(connection.node_ref);
                self.load_left += node.load;
                self.load_left += connection.alpha * node.a[1];
            }
        }

        if self.right_connections.is_empty() {
            self.load_right = 0.0;
        } else {
            self.load_right = self.alpha_right * self.right.a[1];
            for connection in self.right_connections.iter() {
                let node = parent.get_node(connection.node_ref);
                self.load_right += node.load;
                self.load_right += connection.alpha * node.a[0];
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

#[derive(Debug, Clone, Copy)]
pub enum DwgNodeRef {
    Dwg0Left,
    Dwg0Right,
    Dwg1Left,
    Dwg1Right,
    Dwg2Left,
    Dwg2Right,
    Dwg3Left,
    Dwg3Right,
}
