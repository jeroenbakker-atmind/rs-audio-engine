use audio_engine_common::id::{GetID, ID};

use crate::{
    event::Event,
    row::{Row, RowID},
};

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

impl Pattern {
    pub fn init(&mut self, strings: &[&str]) {
        for (string, row) in strings.iter().zip(self.rows.iter_mut()) {
            row.init(string);
        }
        self.rows[strings.len()].event = Some(Event::PatternEnd);
    }
}
