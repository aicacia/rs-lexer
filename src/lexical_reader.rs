use std::io::Read;
use std::convert::From;
use std::hash::Hash;

use super::input::{input_update, Input};
use super::readers::Readers;
use super::token::Token;


pub struct LexicalReader<T>
    where T: Clone + Eq + PartialEq + Hash
{
    pub readers: Readers<T>,
    pub input: Input,
}

impl<'a, T> From<&'a str> for LexicalReader<T>
    where T: Clone + Eq + PartialEq + Hash
{
    #[inline]
    fn from(value: &'a str) -> Self {
        LexicalReader {
            readers: Readers::new(),
            input: From::from(value),
        }
    }
}

impl<'a, T> From<&'a String> for LexicalReader<T>
    where T: Clone + Eq + PartialEq + Hash
{
    #[inline]
    fn from(value: &'a String) -> Self {
        From::from(value.as_str())
    }
}

impl<'a, R, T> From<&'a mut R> for LexicalReader<T>
    where R: Read,
          T: Clone + Eq + PartialEq + Hash
{
    #[inline]
    fn from(value: &'a mut R) -> Self {
        let mut string = String::new();
        value.read_to_string(&mut string).expect("failed to read value");
        From::from(string.as_str())
    }
}

impl<T> Iterator for LexicalReader<T>
    where T: Clone + Eq + PartialEq + Hash
{
    type Item = Token<T>;


    fn next(&mut self) -> Option<Self::Item> {
        if self.input.state().done() {
            None
        } else {
            let mut token = None;
            let mut new_state = None;

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

            token
        }
    }
}
