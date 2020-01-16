use alloc::boxed::Box;
use alloc::vec::Vec;

use core::any::{Any, TypeId};
use core::slice;

use super::{Reader, ReadersBuilder, ReadersLexer};

pub struct Readers<T, E>(Vec<Box<dyn Reader<T, E>>>);

unsafe impl<T, E> Sync for Readers<T, E>
where
  T: Sync,
  E: Sync,
{
}
unsafe impl<T, E> Send for Readers<T, E>
where
  T: Send,
  E: Send,
{
}

impl<T, E> From<ReadersBuilder<T, E>> for Readers<T, E> {
  #[inline]
  fn from(readers_builder: ReadersBuilder<T, E>) -> Readers<T, E> {
    Readers(readers_builder.0)
  }
}

impl<T, E> Readers<T, E>
where
  T: 'static,
  E: 'static,
{
  #[inline]
  pub fn remove<R: 'static + Reader<T, E>>(&mut self) {
    let type_id = TypeId::of::<R>();

    if let Some(index) = self.0.iter().position(|r| (&**r).type_id() == type_id) {
      self.0.remove(index);
    }
  }
}

impl<T, E> Readers<T, E> {
  #[inline]
  pub fn new() -> Self {
    Readers(Vec::new())
  }

  #[inline]
  pub fn add<R: 'static + Reader<T, E>>(&mut self, reader: R) -> &mut Self {
    let index = self
      .0
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
  pub fn get(&self, index: usize) -> Option<&dyn Reader<T, E>> {
    self.0.get(index).map(Box::as_ref)
  }

  #[inline]
  pub fn get_mut(&mut self, index: usize) -> Option<&mut (dyn Reader<T, E> + 'static)> {
    self.0.get_mut(index).map(Box::as_mut)
  }

  #[inline]
  pub fn lexer<'a, I>(&'a self, iter: I) -> ReadersLexer<'a, T, E, I>
  where
    I: Iterator<Item = char>,
  {
    ReadersLexer::new(self, iter)
  }
}

impl<'a, T, E> Readers<T, E>
where
  T: 'a,
  E: 'a,
{
  #[inline]
  pub fn iter(&'a self) -> ReadersIter<'a, T, E> {
    ReadersIter {
      iter: self.0.iter(),
    }
  }
}

impl<'a, T, E> IntoIterator for &'a Readers<T, E>
where
  T: 'a,
  E: 'a,
{
  type Item = &'a dyn Reader<T, E>;
  type IntoIter = ReadersIter<'a, T, E>;

  #[inline(always)]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

pub struct ReadersIter<'a, T, E>
where
  T: 'a,
  E: 'a,
{
  iter: slice::Iter<'a, Box<dyn Reader<T, E>>>,
}

impl<'a, T, E> Iterator for ReadersIter<'a, T, E>
where
  T: 'a,
  E: 'a,
{
  type Item = &'a dyn Reader<T, E>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|reader| &**reader)
  }
}
