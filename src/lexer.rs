use peek_nth::{IteratorExt, PeekableNth};

use super::{Input, ReaderResult, Readers, State};

pub struct Lexer<'a, T, E, I, D = ()>
where
    T: 'a,
    E: 'a,
    I: 'a + IteratorExt<Item = char>,
    D: 'a,
{
    readers: &'a Readers<T, E, D>,
    state: State,
    input: PeekableNth<I>,
    data: &'a mut D,
}

unsafe impl<'a, T, E, I, D> Sync for Lexer<'a, T, E, I, D>
where
    T: 'a + Sync,
    E: 'a + Sync,
    I: 'a + Sync + IteratorExt<Item = char>,
    D: 'a + Sync,
{
}
unsafe impl<'a, T, E, I, D> Send for Lexer<'a, T, E, I, D>
where
    T: 'a + Send,
    E: 'a + Send,
    I: 'a + Send + IteratorExt<Item = char>,
    D: 'a + Send,
{
}

impl<'a, T, E, I, D> From<(&'a Readers<T, E, D>, I, &'a mut D)> for Lexer<'a, T, E, I, D>
where
    T: 'a,
    E: 'a,
    I: 'a + IteratorExt<Item = char>,
    D: 'a,
{
    #[inline(always)]
    fn from((readers, iter, data): (&'a Readers<T, E, D>, I, &'a mut D)) -> Self {
        Lexer {
            readers: readers,
            state: State::new(),
            input: iter.peekable_nth(),
            data: data,
        }
    }
}

impl<'a, T, E, I, D> Lexer<'a, T, E, I, D>
where
    T: 'a,
    E: 'a,
    I: 'a + IteratorExt<Item = char>,
    D: 'a,
{
    #[inline(always)]
    pub fn new(readers: &'a Readers<T, E, D>, iter: I, data: &'a mut D) -> Self {
        From::from((readers, iter, data))
    }
}

impl<'a, T, E, I, D> Iterator for Lexer<'a, T, E, I, D>
where
    T: 'a,
    E: 'a,
    I: 'a + IteratorExt<Item = char>,
    D: 'a,
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

                match reader.read(&mut self.input, &self.state, &mut state, self.data) {
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
