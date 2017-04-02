use collections::vec::Vec;

use super::state::State;


pub trait Input {
    fn read(&self, state: &mut State) -> char;
    fn done(&self, state: &State) -> bool;
    fn can_read(&self, state: &State, offset: usize) -> bool;
    fn peek(&self, state: &State, offset: usize) -> char;
}

impl Input for Vec<char> {

    #[inline]
    fn read(&self, state: &mut State) -> char {
        let ch = self.peek(state, 0);
        state.read(ch == '\n');
        ch
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
    fn peek(&self, state: &State, offset: usize) -> char {
        unsafe {
            *self.get_unchecked(state.index() + offset)
        }
    }
}
