use audio_engine_common::{id::ID, level::Level};

use crate::event::Event;

#[derive(Default, Copy, Clone)]
pub struct Row {
    pub event: Option<Event>,
    pub level: Option<Level>,
}

pub type RowID = ID<Row>;
