// This could be replaced by our ring buffer.
#[derive(Debug, Default, Clone)]
pub struct Delay {
    di: i32,
    d1: i32,
    size: i32,
    cursor: i32,
    x: Vec<f32>,
    y: Vec<f32>,
}

impl Delay {
    pub fn new(di: i32) -> Delay {
        let mut result = Delay::default();
        result.init(di);
        result
    }

    fn init(&mut self, di: i32) {
        self.size = ((2 * di) as usize).next_power_of_two() as i32;

        self.x = vec![0.0; self.size as usize];
        self.y = vec![0.0; self.size as usize];

        self.cursor = 0;
        self.di = di;
        self.d1 = (self.size - di) & (self.size - 1);
    }

    pub fn delay(&mut self, in_value: f32) -> f32 {
        let result = self.x[self.d1 as usize];
        self.y[self.cursor as usize] = result;
        self.x[self.cursor as usize] = in_value;
        self.d1 = (self.d1 + 1) & (self.size - 1);
        self.cursor = (self.cursor + 1) & (self.size - 1);
        result
    }
}
