

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    pub index: usize,
    pub row: u64,
    pub col: u64,
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
    pub fn done(&self) -> bool {
        self.index >= self.len
    }

    #[inline(always)]
    pub fn has(&self, offset: usize) -> bool {
        (self.index + offset) < self.len
    }
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
