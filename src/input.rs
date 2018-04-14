use alloc::string::String;
use alloc::vec::Vec;

use peek_nth::PeekableNth;

use super::{Lines, State};

pub trait Input {
    fn peek(&mut self, state: &State, offset: usize) -> Option<char>;
    fn lines<'a>(&'a mut self, state: &'a mut State) -> Lines<'a>;

    #[inline]
    fn read(&mut self, state: &mut State) -> Option<char> {
        match self.peek(state, 0) {
            Some(ch) => {
                state.read(ch == '\n');
                Some(ch)
            }
            None => None,
        }
    }

    #[inline]
    fn read_offset(&mut self, state: &mut State, offset: usize) -> usize {
        let mut read = 0;

        for _ in 0..offset {
            if self.read(state).is_none() {
                break;
            } else {
                read += 1;
            }
        }

        read
    }

    #[inline]
    fn skip_line(&mut self, state: &mut State) {
        if !self.is_done(state) {
            while let Some(ch) = self.read(state) {
                if ch == '\n' {
                    break;
                }
            }
        }
    }
    #[inline]
    fn peek_line(&mut self, state: &State) -> Option<String> {
        if self.is_done(state) {
            None
        } else {
            let mut string = String::new();
            let mut index = 0;

            while let Some(ch) = self.peek(state, index) {
                if ch != '\n' {
                    index += 1;
                    string.push(ch);
                } else {
                    break;
                }
            }

            Some(string)
        }
    }
    #[inline]
    fn read_line(&mut self, state: &mut State) -> Option<String> {
        if self.is_done(state) {
            None
        } else {
            let mut string = String::new();

            while let Some(ch) = self.read(state) {
                if ch != '\n' {
                    string.push(ch);
                } else {
                    break;
                }
            }

            Some(string)
        }
    }

    #[inline]
    fn read_whitespaces(&mut self, state: &mut State) -> Option<String> {
        if self.is_done(state) {
            None
        } else {
            let mut string = String::new();

            while let Some(ch) = self.peek(state, 0) {
                if ch.is_whitespace() {
                    self.read(state);
                    string.push(ch);
                } else {
                    break;
                }
            }

            if string.is_empty() {
                None
            } else {
                Some(string)
            }
        }
    }

    #[inline]
    fn skip_whitespaces(&mut self, state: &mut State) {
        if !self.is_done(state) {
            while let Some(ch) = self.peek(state, 0) {
                if ch.is_whitespace() {
                    self.read(state);
                }
            }
        }
    }

    #[inline]
    fn is_done(&mut self, state: &State) -> bool {
        self.peek(state, 0).is_none()
    }

    #[inline]
    fn can_read(&mut self, state: &State, offset: usize) -> bool {
        self.peek(state, offset).is_some()
    }
}

impl<I> Input for PeekableNth<I>
where
    I: Iterator<Item = char>,
{
    #[inline]
    fn peek(&mut self, state: &State, offset: usize) -> Option<char> {
        self.peek_nth(state.index() + offset).map(Clone::clone)
    }
    #[inline]
    fn lines<'a>(&'a mut self, state: &'a mut State) -> Lines<'a> {
        Lines::new(self, state)
    }
}

impl Input for Vec<char> {
    #[inline]
    fn peek(&mut self, state: &State, offset: usize) -> Option<char> {
        self.get(state.index() + offset).map(Clone::clone)
    }
    #[inline]
    fn lines<'a>(&'a mut self, state: &'a mut State) -> Lines<'a> {
        Lines::new(self, state)
    }
}

impl Input for String {
    #[inline]
    fn peek(&mut self, state: &State, offset: usize) -> Option<char> {
        self.chars().nth(state.index() + offset)
    }
    #[inline]
    fn lines<'a>(&'a mut self, state: &'a mut State) -> Lines<'a> {
        Lines::new(self, state)
    }
}

impl<'s> Input for &'s str {
    #[inline]
    fn peek(&mut self, state: &State, offset: usize) -> Option<char> {
        self.chars().nth(state.index() + offset)
    }
    #[inline]
    fn lines<'a>(&'a mut self, state: &'a mut State) -> Lines<'a> {
        Lines::new(self, state)
    }
}
