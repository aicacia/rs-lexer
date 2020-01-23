use alloc::boxed::Box;
use alloc::vec::Vec;
use core::ops::{Deref, DerefMut};
use core::slice;

use peek_nth::{IteratorExt, PeekableNth};

use super::{Lexer, Reader, ReadersBuilder};

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

impl<T, E> Readers<T, E> {
  #[inline]
  pub fn new() -> Self {
    Readers(Vec::new())
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
  pub fn tokens<'a, I>(&'a self, iter: I) -> PeekableNth<Lexer<'a, T, E, I>>
  where
    I: Iterator<Item = char>,
  {
    Lexer::new(self, iter).peekable_nth()
  }
}

impl<T, E> Deref for Readers<T, E> {
  type Target = Vec<Box<dyn Reader<T, E>>>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl<T, E> DerefMut for Readers<T, E> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
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
  #[inline]
  pub fn iter_mut(&'a mut self) -> ReadersIterMut<'a, T, E> {
    ReadersIterMut {
      iter: self.0.iter_mut(),
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

impl<'a, T, E> IntoIterator for &'a mut Readers<T, E>
where
  T: 'a,
  E: 'a,
{
  type Item = &'a mut (dyn Reader<T, E> + 'static);
  type IntoIter = ReadersIterMut<'a, T, E>;

  #[inline(always)]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
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

pub struct ReadersIterMut<'a, T, E>
where
  T: 'a,
  E: 'a,
{
  iter: slice::IterMut<'a, Box<dyn Reader<T, E>>>,
}

impl<'a, T, E> Iterator for ReadersIterMut<'a, T, E>
where
  T: 'a,
  E: 'a,
{
  type Item = &'a mut (dyn Reader<T, E> + 'static);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|reader| &mut **reader)
  }
}
