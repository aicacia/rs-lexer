use alloc::vec::Vec;

use core::convert::From;

use super::input::Input;
use super::state::State;
use super::readers::Readers;


pub struct Lexer<T, I: Input> {
    pub readers: Readers<T>,
    pub state: State,
    pub input: I,
}

impl<T, I: Input> Lexer<T, I> {

    #[inline]
    pub fn new(input: I) -> Self {
        Lexer {
            readers: Readers::new(),
            state: State::new(),
            input: input,
        }
    }
}

impl<T, I> From<I> for Lexer<T, I>
    where I: Input
{
    #[inline(always)]
    fn from(value: I) -> Self {
        Self::new(value)
    }
}

impl<'a, T> From<&'a str> for Lexer<T, Vec<char>> {
    #[inline]
    fn from(value: &'a str) -> Self {
        Self::new(value.chars().collect())
    }
}

impl<T, I: Input> Iterator for Lexer<T, I> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.input.done(&self.state) {
            None
        } else {
            let mut token = None;
            let mut new_state = None;
            let orig_state = self.state.clone();

            for reader in self.readers.iter() {
                let mut state = orig_state.clone();

                match reader.read(&self.input, &self.state, &mut state) {
                    Some(t) => {
                        token = Some(t);
                        new_state = Some(state);
                        break;
                    },
                    None => (),
                }
            }

            if let Some(ref state) = new_state {
                self.state.clone_from(state);
            }

            assert!(
                orig_state.index() != self.state.index() || self.input.done(&self.state),
                "Lexer: No reader was able to read at {:?}",
                orig_state
            );

            token
        }
    }
}
