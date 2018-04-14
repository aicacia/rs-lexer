use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{Reader, Readers};

pub struct ReadersBuilder<T, E, D>(pub(crate) Vec<Box<Reader<T, E, D>>>);

unsafe impl<T, E, D> Sync for ReadersBuilder<T, E, D>
where
    T: Sync,
    E: Sync,
{
}
unsafe impl<T, E, D> Send for ReadersBuilder<T, E, D>
where
    T: Send,
    E: Send,
{
}

impl<T, E, D> ReadersBuilder<T, E, D> {
    #[inline]
    pub fn new() -> Self {
        ReadersBuilder(Vec::new())
    }

    #[inline]
    pub fn add<R: 'static + Reader<T, E, D>>(mut self, reader: R) -> Self {
        let index = self.0
            .iter()
            .position(|r| reader.priority() <= r.priority());
        let boxed_reader = Box::new(reader);

        if let Some(index) = index {
            self.0.insert(index, boxed_reader);
        } else {
            self.0.push(boxed_reader);
        }

        self
    }

    #[inline]
    pub fn build(self) -> Readers<T, E, D> {
        Readers::from(self)
    }
}
