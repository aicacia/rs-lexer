use chars_input::{Input, State};

use super::{Readers, ReaderResult};


pub struct Lexer<'a, T, E, I>
    where T: 'a,
          E: 'a,
          I: 'a + Input,
{
    readers: &'a Readers<T, E>,
    state: State,
    input: I,
}

unsafe impl<'a, T, E, I> Sync for Lexer<'a, T, E, I>
    where T: 'a + Sync,
          E: 'a + Sync,
          I: 'a + Sync + Input,
{}
unsafe impl<'a, T, E, I> Send for Lexer<'a, T, E, I>
    where T: 'a + Send,
          E: 'a + Send,
          I: 'a + Send + Input,
{}

impl<'a, T, E, I> From<(&'a Readers<T, E>, I)> for Lexer<'a, T, E, I>
    where T: 'a,
          E: 'a,
          I: 'a + Input,
{
    #[inline(always)]
    fn from((readers, input): (&'a Readers<T, E>, I)) -> Self {
        Lexer {
            readers: readers,
            state: State::new(),
            input: input,
        }
    }
}

impl<'a, T, E, I> Lexer<'a, T, E, I>
    where T: 'a,
          E: 'a,
          I: 'a + Input,
{
    #[inline(always)]
    pub fn new(readers: &'a Readers<T, E>, input: I) -> Self {
        From::from((readers, input))
    }
}

impl<'a, T, E, I> Iterator for Lexer<'a, T, E, I>
    where T: 'a,
          E: 'a,
          I: 'a + Input,
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
                    },
                    ReaderResult::Err(e) => {
                        return Some(Err(e));
                    },
                    ReaderResult::Empty => {
                        new_state = Some(state);
                        is_empty = true;
                        break;
                    },
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
