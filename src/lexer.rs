use peek_nth::{IteratorExt, PeekableNth};

use super::{Input, ReaderResult, Readers, State};

pub struct Lexer<'a, T, E, I>
where
    T: 'a,
    E: 'a,
    I: 'a + IteratorExt<Item = char>,
{
    readers: &'a Readers<T, E>,
    state: State,
    input: PeekableNth<I>,
}

unsafe impl<'a, T, E, I> Sync for Lexer<'a, T, E, I>
where
    T: 'a + Sync,
    E: 'a + Sync,
    I: 'a + Sync + IteratorExt<Item = char>,
{
}
unsafe impl<'a, T, E, I> Send for Lexer<'a, T, E, I>
where
    T: 'a + Send,
    E: 'a + Send,
    I: 'a + Send + IteratorExt<Item = char>,
{
}

impl<'a, T, E, I> From<(&'a Readers<T, E>, I)> for Lexer<'a, T, E, I>
where
    T: 'a,
    E: 'a,
    I: 'a + IteratorExt<Item = char>,
{
    #[inline(always)]
    fn from((readers, iter): (&'a Readers<T, E>, I)) -> Self {
        Lexer {
            readers: readers,
            state: State::new(),
            input: iter.peekable_nth(),
        }
    }
}

impl<'a, T, E, I> Lexer<'a, T, E, I>
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

impl<'a, T, E, I> Iterator for Lexer<'a, T, E, I>
where
    T: 'a,
    E: 'a,
    I: 'a + IteratorExt<Item = char>,
{
    type Item = Result<T, E>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_done(&self.state) {
            None
        } else {
            let mut token = None;
            let mut new_state = None;
            let mut is_empty = false;
            let orig_state = self.state.clone();

            for reader in self.readers {
                let mut state = orig_state.clone();

                match reader.read(&mut self.input, &self.state, &mut state) {
                    ReaderResult::Some(t) => {
                        token = Some(Ok(t));
                        new_state = Some(state);
                        break;
                    }
                    ReaderResult::Err(e) => {
                        return Some(Err(e));
                    }
                    ReaderResult::Empty => {
                        new_state = Some(state);
                        is_empty = true;
                        break;
                    }
                    ReaderResult::None => (),
                }
            }

            if let Some(ref state) = new_state {
                self.state.clone_from(state);
            }

            if is_empty {
                self.next()
            } else {
                debug_assert!(
                    orig_state.index() != self.state.index() || self.input.is_done(&self.state),
                    "Lexer: No reader was able to read at {:?}",
                    orig_state
                );
                token
            }
        }
    }
}
