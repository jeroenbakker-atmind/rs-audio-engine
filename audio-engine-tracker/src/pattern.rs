use audio_engine_common::id::{GetID, ID};

use crate::row::{Row, RowID};

#[derive(Copy, Clone)]
pub struct Pattern {
    pub rows: [Row; 255],
}

impl Default for Pattern {
    fn default() -> Self {
        Self {
            rows: [Row::default(); 255],
        }
    }
}

pub type PatternID = ID<Pattern>;

impl GetID<Row> for Pattern {
    fn get(&self, id: RowID) -> Option<&Row> {
        match id {
            RowID::Index(index) => Some(&self.rows[index as usize]),
            _ => None,
        }
    }
}
