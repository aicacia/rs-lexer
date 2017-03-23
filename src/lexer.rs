use core::convert::From;
use core::hash::Hash;

use super::input::{input_update, Input};
use super::readers::Readers;
use super::token::Token;


pub struct Lexer<T>
    where T: Clone + Eq + PartialEq + Hash
{
    pub readers: Readers<T>,
    pub input: Input,
}

impl<'a, T> From<&'a str> for Lexer<T>
    where T: Clone + Eq + PartialEq + Hash
{
    #[inline(always)]
    fn from(value: &'a str) -> Self {
        Lexer {
            readers: Readers::new(),
            input: From::from(value),
        }
    }
}

impl<T> Iterator for Lexer<T>
    where T: Clone + Eq + PartialEq + Hash
{
    type Item = Token<T>;


    fn next(&mut self) -> Option<Self::Item> {
        if self.input.done(self.input.state()) {
            None
        } else {
            let mut token = None;
            let mut new_state = None;
            let orig_index = self.input.state().index();

            for reader in self.readers.iter() {
                let mut state = self.input.state().clone();

                match reader.read(&self.input, &mut state) {
                    Some(t) => {
                        token = Some(t);
                        new_state = Some(state);
                        break;
                    },
                    None => (),
                }
            }

            if let Some(ref state) = new_state {
                input_update(&mut self.input, state);
            }

            assert!(
                orig_index != self.input.state().index() ||
                self.input.done(self.input.state()),
                "Lexer: No reader was able to read at index {:?}",
                orig_index
            );

            token
        }
    }
}
