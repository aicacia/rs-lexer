use alloc::vec::Vec;

use super::State;


pub trait Input {

    fn peek(&self, state: &State, offset: usize) -> Option<char>;

    #[inline]
    fn read(&self, state: &mut State) -> Option<char> {
        match self.peek(state, 0) {
            Some(ch) => {
                state.read(ch == '\n');
                Some(ch)
            },
            None => None,
        }
    }

    #[inline(always)]
    fn done(&self, state: &State) -> bool {
        self.peek(state, 0).is_none()
    }

    #[inline(always)]
    fn can_read(&self, state: &State, offset: usize) -> bool {
        self.peek(state, offset).is_some()
    }
}

impl<'a> Input for &'a [char] {
    #[inline]
    fn peek(&self, state: &State, offset: usize) -> Option<char> {
        let index = state.index() + offset;

        if index < self.len() {
            Some(self[state.index() + offset])
        } else {
            None
        }
    }
}

impl<'a> Input for &'a Vec<char> {
    #[inline(always)]
    fn peek(&self, state: &State, offset: usize) -> Option<char> {
        (&***self).peek(state, offset)
    }
}

impl Input for Vec<char> {
    #[inline(always)]
    fn peek(&self, state: &State, offset: usize) -> Option<char> {
        (&**self).peek(state, offset)
    }
}
