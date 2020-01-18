use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{Reader, Readers};

pub struct ReadersBuilder<T, E>(pub(crate) Vec<Box<dyn Reader<T, E>>>);

unsafe impl<T, E> Sync for ReadersBuilder<T, E>
where
  T: Sync,
  E: Sync,
{
}
unsafe impl<T, E> Send for ReadersBuilder<T, E>
where
  T: Send,
  E: Send,
{
}

impl<T, E> ReadersBuilder<T, E> {
  #[inline]
  pub fn new() -> Self {
    ReadersBuilder(Vec::new())
  }

  #[inline]
  pub fn add<R: 'static + Reader<T, E>>(mut self, reader: R) -> Self {
    self.0.push(Box::new(reader));
    self
  }

  #[inline]
  pub fn build(self) -> Readers<T, E> {
    Readers::from(self)
  }
}
