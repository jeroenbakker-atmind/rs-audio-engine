use audio_engine_common::id::ID;

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

pub type PatternID = ID;

impl Pattern {
    fn get_row(&self, id: RowID) -> Option<&Row> {
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

    pub fn count_rows(&self) -> u32 {
        let mut row_count = 0;
        for row in &self.rows {
            if let Some(Event::PatternEnd) = row.event {
                return row_count;
            }
            row_count += 1;
        }
        row_count
    }
}
