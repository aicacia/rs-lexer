use super::{Input, State, Readers, ReaderOption};


pub struct Lexer<'a, T, I>
    where T: 'a,
          I: 'a + Input,
{
    readers: &'a Readers<T>,
    state: State,
    input: I,
}

impl<'a, T, I> From<(&'a Readers<T>, I)> for Lexer<'a, T, I>
    where T: 'a,
          I: 'a + Input,
{
    #[inline(always)]
    fn from((readers, input): (&'a Readers<T>, I)) -> Self {
        Lexer {
            readers: readers,
            state: State::new(),
            input: input,
        }
    }
}

impl<'a, T, I> Lexer<'a, T, I>
    where T: 'a,
          I: 'a + Input,
{
    #[inline(always)]
    pub fn new(readers: &'a Readers<T>, input: I) -> Self {
        From::from((readers, input))
    }
}

impl<'a, T, I> Iterator for Lexer<'a, T, I>
    where T: 'a,
          I: 'a + Input,
{
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.done(&self.state) {
            None
        } else {
            let mut token = None;
            let mut new_state = None;
            let mut is_empty = false;
            let orig_state = self.state.clone();

            for reader in self.readers {
                let mut state = orig_state.clone();

                match reader.read(&mut self.input, &self.state, &mut state) {
                    ReaderOption::Some(t) => {
                        token = Some(t);
                        new_state = Some(state);
                        break;
                    },
                    ReaderOption::Empty => {
                        new_state = Some(state);
                        is_empty = true;
                        break;
                    },
                    ReaderOption::None => (),
                }
            }

            if let Some(ref state) = new_state {
                self.state.clone_from(state);
            }

            if is_empty {
                self.next()
            } else {
                assert!(
                    orig_state.index() != self.state.index() || self.input.done(&self.state),
                    "Lexer: No reader was able to read at {:?}",
                    orig_state
                );
                token
            }
        }
    }
}
