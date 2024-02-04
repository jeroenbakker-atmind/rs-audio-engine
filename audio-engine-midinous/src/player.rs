use crate::{link_index::LinkIndex, traveler::Traveler};

pub struct Player {
    outgoing_link_lookup: Vec<Vec<LinkIndex>>,
    travelers: Vec<Traveler>,
}
