#[derive(Default, Copy, Clone, Debug, PartialEq)]
pub enum ID {
    #[default]
    NotSet,
    Index(u8),
}

impl From<u8> for ID {
    fn from(index: u8) -> Self {
        Self::Index(index)
    }
}
