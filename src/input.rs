use core::ops::Index;

use data_structure_traits::*;

use super::state::State;


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

impl<T> Input for T
    where T: Collection +
             Index<usize, Output=char>
{
    #[inline(always)]
    fn peek(&self, state: &State, offset: usize) -> Option<char> {
        let index = state.index() + offset;

        if index < self.len() {
            Some(self[state.index() + offset])
        } else {
            None
        }
    }
}
