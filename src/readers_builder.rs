use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{Reader, Readers};


pub struct ReadersBuilder<T, E> {
    pub(crate) vec: Vec<Box<Reader<T, E>>>,
}

impl<T, E> ReadersBuilder<T, E> {

    #[inline(always)]
    pub fn new() -> Self {
        ReadersBuilder {
            vec: Vec::new(),
        }
    }

    #[inline]
    pub fn add<R: 'static + Reader<T, E>>(mut self, reader: R) -> Self {
        self.vec.push(Box::new(reader));
        self
    }

    #[inline]
    pub fn build(mut self) -> Readers<T, E> {
        self.vec.sort_by(|a, b| a.priority().cmp(&b.priority()));
        Readers::from(self)
    }
}
