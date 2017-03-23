use collections::vec::Vec;

use core::convert::From;
use core::ops::Deref;

use super::state::{state_read, state_update, State};
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
        TokenMeta::new(
            self.state.index() as u64,
            state.index() as u64,
            self.state.col(),
            state.col(),
            self.state.row(),
            state.row()
        )
    }

    #[inline]
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
    pub fn done(&self, state: &State) -> bool {
        state.index() >= self.input.len()
    }

    #[inline(always)]
    pub fn has_char_at(&self, state: &State, offset: usize) -> bool {
        (state.index() + offset) < self.input.len()
    }

    #[inline(always)]
    pub fn char_at(&self, state: &State, offset: usize) -> char {
        unsafe {
            *self.input.get_unchecked(state.index() + offset)
        }
    }
}

#[inline(always)]
pub fn input_update<'a>(input: &'a mut Input, state: &State) {
    state_update(&mut input.state, state);
}
