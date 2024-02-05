use crate::link_index::LinkIndex;

#[derive(Copy, Clone)]
pub struct Traveler {
    pub link: LinkIndex,
    pub distance_traveled: f32,
}

impl Default for Traveler {
    fn default() -> Self {
        Self {
            link: 0.into(),
            distance_traveled: 0.0,
        }
    }
}
