

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    index: usize,
    row: u64,
    col: u64,
    len: usize,
}

impl State {

    #[inline(always)]
    pub fn new(len: usize) -> Self {
        State {
            index: 0usize,
            row: 1u64,
            col: 1u64,
            len: len,
        }
    }

    #[inline(always)]
    pub fn index(&self) -> usize { self.index }
    #[inline(always)]
    pub fn row(&self) -> u64 { self.row }
    #[inline(always)]
    pub fn col(&self) -> u64 { self.col }
    #[inline(always)]
    pub fn len(&self) -> usize { self.len }
}

pub fn state_read<'a>(state: &'a mut State, is_newline: bool) {
    if is_newline {
        state.row += 1;
        state.col = 1;
    } else if state.index != 0 {
        state.col += 1;
    }

    if state.index < state.len {
        state.index += 1;
    }
}

#[inline]
pub fn state_update<'a>(state: &'a mut State, other: &State) {
    state.index = other.index;
    state.row = other.row;
    state.col = other.col;
}
