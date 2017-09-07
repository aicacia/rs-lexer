use core::ops::Index;

use data_structure_traits::*;

use super::state::State;


pub trait Input {
    fn read(&self, state: &mut State) -> Option<char>;
    fn done(&self, state: &State) -> bool;
    fn can_read(&self, state: &State, offset: usize) -> bool;
    fn peek(&self, state: &State, offset: usize) -> Option<char>;
}

impl<T> Input for T
    where T: Collection +
             Index<usize, Output = char> +
{
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
        state.index() >= self.len()
    }

    #[inline(always)]
    fn can_read(&self, state: &State, offset: usize) -> bool {
        (state.index() + offset) < self.len()
    }

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
