use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{Reader, Readers};


pub struct ReadersBuilder<T> {
    pub(crate) vec: Vec<Box<Reader<T>>>,
}

impl<T> ReadersBuilder<T> {

    #[inline(always)]
    pub fn new() -> Self {
        ReadersBuilder {
            vec: Vec::new(),
        }
    }

    #[inline]
    pub fn add<R: 'static + Reader<T>>(mut self, reader: R) -> Self {
        self.vec.push(Box::new(reader));
        self
    }

    #[inline]
    pub fn build(mut self) -> Readers<T> {
        self.vec.sort_by(|a, b| a.priority().cmp(&b.priority()));
        Readers::from(self)
    }
}
