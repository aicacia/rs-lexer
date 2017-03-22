use std::io::Read;
use std::convert::From;
use std::ops::Deref;

use super::state::{state_read, State};
use super::token::TokenMeta;


pub struct Input {
    state: State,
    input: Vec<char>,
}

impl<'a> From<&'a str> for Input {
    #[inline]
    fn from(value: &'a str) -> Self {
        let input: Vec<char> = value.chars().collect();

        Input {
            state: State::new(input.len()),
            input: input,
        }
    }
}

impl<'a> From<&'a String> for Input {
    #[inline]
    fn from(value: &'a String) -> Self {
        From::from(value.as_str())
    }
}

impl<'a, R> From<&'a mut R> for Input
    where R: Read
{
    #[inline]
    fn from(value: &'a mut R) -> Self {
        let mut string = String::new();
        value.read_to_string(&mut string).expect("failed to read value");
        From::from(string.as_str())
    }
}

impl Deref for Input {
    type Target = [char];

    fn deref(&self) -> &Self::Target {
        &*self.input
    }
}

impl Input {

    #[inline(always)]
    pub fn state(&self) -> &State { &self.state }

    #[inline(always)]
    pub fn new_state_meta(&self, state: &State) -> TokenMeta {
        TokenMeta::new(self.state.col, state.col, self.state.row, state.row)
    }

    pub fn read(&self, state: &mut State) -> char {
        let mut is_newline = false;

        let ch = self.char_at(state, 0);

        if ch == '\n' {
            is_newline = true;
        }

        state_read(state, is_newline);

        ch
    }

    #[inline(always)]
    pub fn has_char_at(&self, state: &State, offset: usize) -> bool {
        state.has(offset)
    }

    #[inline(always)]
    pub fn char_at(&self, state: &State, offset: usize) -> char {
        unsafe {
            *self.input.get_unchecked(state.index + offset)
        }
    }
}

#[inline]
pub fn input_update<'a>(input: &'a mut Input, state: &State) {
    input.state.col = state.col;
    input.state.row = state.row;
    input.state.index = state.index;
}
