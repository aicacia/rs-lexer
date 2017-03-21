use std::io::Read;
use std::convert::From;
use std::hash::Hash;

use super::reader::Reader;
use super::state::{state_read, State};
use super::token::{Token, TokenMeta};


pub struct Lexer<T: Clone + Eq + PartialEq + Hash> {
    state: State,
    readers: Vec<Box<Reader<T>>>,
    input: Vec<char>,
}

impl<'a, T: Clone + Eq + PartialEq + Hash> From<&'a str> for Lexer<T> {
    #[inline]
    fn from(value: &'a str) -> Self {
        let input: Vec<char> = value.chars().collect();

        Lexer {
            state: State::new(input.len()),
            readers: Vec::new(),
            input: input,
        }
    }
}

impl<'a, T: Clone + Eq + PartialEq + Hash> From<&'a String> for Lexer<T> {
    #[inline]
    fn from(value: &'a String) -> Self {
        From::from(value.as_str())
    }
}

impl<'a, R: Read, T: Clone + Eq + PartialEq + Hash> From<&'a mut R> for Lexer<T> {
    #[inline]
    fn from(value: &'a mut R) -> Self {
        let mut string = String::new();
        value.read_to_string(&mut string).expect("failed to read value");
        From::from(string.as_str())
    }
}

impl<T: Clone + Eq + PartialEq + Hash> Lexer<T> {

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
    pub fn meta(&self, state: &State) -> TokenMeta {
        TokenMeta::new(self.col(), state.col, self.row(), state.row)
    }

    #[inline]
    pub fn add_reader<R: 'static + Reader<T>>(&mut self, reader: R) -> &mut Self {
        self.readers.push(Box::new(reader));
        self
    }

    #[inline]
    pub fn sort_readers(&mut self) -> &mut Self {
        self.readers.sort_by(|a, b| a.priority().cmp(&b.priority()));
        self
    }

    #[inline(always)]
    pub fn state(&self) -> &State { &self.state }
    #[inline(always)]
    pub fn row(&self) -> u64 {self.state.row }
    #[inline(always)]
    pub fn col(&self) -> u64 {self.state.col }
    #[inline(always)]
    pub fn index(&self) -> usize {self.state.index }

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

    #[inline]
    fn update(&mut self, state: &State) {
        self.state.col = state.col;
        self.state.row = state.row;
        self.state.index = state.index;
    }
}

impl<T: Clone + Eq + PartialEq + Hash> Iterator for Lexer<T> {
    type Item = Token<T>;


    fn next(&mut self) -> Option<Self::Item> {
        if self.state.done() {
            None
        } else {
            let mut token = None;
            let mut new_state = None;

            for reader in self.readers.iter() {
                let mut state = self.state.clone();

                match reader.read(&self, &mut state) {
                    Some(t) => {
                        token = Some(t);
                        new_state = Some(state);
                        break;
                    },
                    None => (),
                }
            }

            if let Some(ref state) = new_state {
                self.update(state);
            }

            token
        }
    }
}
