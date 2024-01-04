// This could be replaced by our ring buffer.
#[derive(Debug, Default, Clone)]
pub struct Delay {
    di: i32,
    d1: i32,
    size: i32,
    mask: i32,
    cursor: i32,
    x: Vec<f32>,
    y: Vec<f32>,
}

// TODO: Delay::new(di)
pub fn init_delay(c: &mut Delay, di: i32) {
    c.size = 2 * di;
    // TODO: weird way to init something that can be done by bitshifting
    let mut p = 0;
    while c.size != 0 {
        c.size /= 2;
        p += 1;
    }

    c.size = 1;
    while p != 0 {
        c.size *= 2;
        p -= 1;
    }

    c.mask = c.size - 1;
    c.x = vec![0.0; c.size as usize];
    c.y = vec![0.0; c.size as usize];

    c.cursor = 0;
    c.di = di;
    c.d1 = (c.size - di) % c.size;
}

pub fn delay(in_value: f32, c: &mut Delay) -> f32 {
    let result = c.x[c.d1 as usize];
    c.y[c.cursor as usize] = result;
    c.x[c.cursor as usize] = in_value;
    c.d1 = (c.d1 + 1) & c.mask;
    c.cursor = (c.cursor + 1) & c.mask;
    result
}
