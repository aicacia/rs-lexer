use collections::vec::Vec;

use core::convert::From;

use super::input::Input;
use super::state::State;
use super::readers::Readers;


pub struct Lexer<T, I: Input> {
    pub readers: Readers<T>,
    pub state: State,
    pub input: I,
}

impl<'a, T> From<&'a str> for Lexer<T, Vec<char>> {
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Lexer {
            readers: Readers::new(),
            state: State::new(),
            input: value.chars().collect(),
        }
    }
}

impl<T, I: Input> Iterator for Lexer<T, I> {
    type Item = T;


    fn next(&mut self) -> Option<Self::Item> {
        if self.input.done(&self.state) {
            None
        } else {
            let mut token = None;
            let mut new_state = None;
            let orig_index = self.state.index();

            for reader in self.readers.iter() {
                let mut state = self.state.clone();

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
                self.state.update(state);
            }

            assert!(
                orig_index != self.state.index() ||
                self.input.done(&self.state),
                "Lexer: No reader was able to read at index {:?}",
                orig_index
            );

            token
        }
    }
}
