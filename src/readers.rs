use alloc::{boxed::Box, vec::Vec};
use core::ops::{Deref, DerefMut};

use peek_nth::{IteratorExt, PeekableNth};

use super::{read, Reader, ReadersBuilder, State};

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
    Readers::from(readers_builder.0)
  }
}

impl<T, E> From<Vec<Box<dyn Reader<T, E>>>> for Readers<T, E> {
  #[inline]
  fn from(vec: Vec<Box<dyn Reader<T, E>>>) -> Readers<T, E> {
    Readers(vec)
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
  pub fn read<'a, I>(&'a self, iter: I) -> PeekableNth<TokenIter<'a, T, E, I>>
  where
    I: Iterator<Item = char>,
  {
    TokenIter::new(self, iter).peekable_nth()
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

pub struct TokenIter<'a, T, E, I>
where
  T: 'a,
  E: 'a,
  I: 'a + IteratorExt<Item = char>,
{
  readers: &'a Readers<T, E>,
  state: State,
  input: PeekableNth<I>,
}

unsafe impl<'a, T, E, I> Sync for TokenIter<'a, T, E, I>
where
  T: 'a + Sync,
  E: 'a + Sync,
  I: 'a + Sync + IteratorExt<Item = char>,
{
}
unsafe impl<'a, T, E, I> Send for TokenIter<'a, T, E, I>
where
  T: 'a + Send,
  E: 'a + Send,
  I: 'a + Send + IteratorExt<Item = char>,
{
}

impl<'a, T, E, I> From<(&'a Readers<T, E>, I)> for TokenIter<'a, T, E, I>
where
  T: 'a,
  E: 'a,
  I: 'a + IteratorExt<Item = char>,
{
  #[inline(always)]
  fn from((readers, iter): (&'a Readers<T, E>, I)) -> Self {
    Self::new(readers, iter)
  }
}

impl<'a, T, E, I> TokenIter<'a, T, E, I>
where
  T: 'a,
  E: 'a,
  I: 'a + IteratorExt<Item = char>,
{
  #[inline(always)]
  pub fn new(readers: &'a Readers<T, E>, iter: I) -> Self {
    TokenIter {
      readers: readers,
      state: State::new(),
      input: iter.peekable_nth(),
    }
  }
}

impl<'a, T, E, I> Iterator for TokenIter<'a, T, E, I>
where
  T: 'a,
  E: 'a,
  I: 'a + IteratorExt<Item = char>,
{
  type Item = Result<T, E>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    read(self.readers, &mut self.input, &mut self.state)
  }
}
