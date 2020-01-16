use peek_nth::{IteratorExt, PeekableNth};

use super::{next, Readers, State};

pub struct ReadersLexer<'a, T, E, I>
where
  T: 'a,
  E: 'a,
  I: 'a + IteratorExt<Item = char>,
{
  readers: &'a Readers<T, E>,
  state: State,
  input: PeekableNth<I>,
}

unsafe impl<'a, T, E, I> Sync for ReadersLexer<'a, T, E, I>
where
  T: 'a + Sync,
  E: 'a + Sync,
  I: 'a + Sync + IteratorExt<Item = char>,
{
}
unsafe impl<'a, T, E, I> Send for ReadersLexer<'a, T, E, I>
where
  T: 'a + Send,
  E: 'a + Send,
  I: 'a + Send + IteratorExt<Item = char>,
{
}

impl<'a, T, E, I> From<(&'a Readers<T, E>, I)> for ReadersLexer<'a, T, E, I>
where
  T: 'a,
  E: 'a,
  I: 'a + IteratorExt<Item = char>,
{
  #[inline(always)]
  fn from((readers, iter): (&'a Readers<T, E>, I)) -> Self {
    ReadersLexer {
      readers: readers,
      state: State::new(),
      input: iter.peekable_nth(),
    }
  }
}

impl<'a, T, E, I> ReadersLexer<'a, T, E, I>
where
  T: 'a,
  E: 'a,
  I: 'a + IteratorExt<Item = char>,
{
  #[inline(always)]
  pub fn new(readers: &'a Readers<T, E>, iter: I) -> Self {
    From::from((readers, iter))
  }
}

impl<'a, T, E, I> Iterator for ReadersLexer<'a, T, E, I>
where
  T: 'a,
  E: 'a,
  I: 'a + IteratorExt<Item = char>,
{
  type Item = Result<T, E>;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    next(self.readers, &mut self.input, &mut self.state)
  }
}
