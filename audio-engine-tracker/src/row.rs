use audio_engine_common::{id::ID, level::Level};

use crate::event::Event;

#[derive(Default, Copy, Clone)]
pub struct Row {
    pub event: Option<Event>,
    pub level: Option<Level>,
}

pub type RowID = ID<Row>;

impl Row {
    pub fn init(&mut self, string: &str) {
        //"--- -- --"

        //"C#4 01 80"
        //"--- -- 50" //only chnage level
        //"REL -- --"
        //"OFF -- --"

    }
}
