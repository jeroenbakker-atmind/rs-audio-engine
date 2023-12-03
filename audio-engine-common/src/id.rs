use std::marker::PhantomData;

#[derive(Default, Copy, Clone, Debug)]
pub enum ID<T> {
    #[default]
    NotSet,
    Index(u8),
    _PhantomData(PhantomData<T>),
}

impl<T> From<u8> for ID<T> {
    fn from(index: u8) -> Self {
        Self::Index(index)
    }
}

pub trait GetID<T> {
    fn get(&self, id: ID<T>) -> Option<&T>;
}
