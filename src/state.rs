

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct State {
    index: usize,
    row: u64,
    col: u64,
}

impl State {

    #[inline(always)]
    pub(crate) fn new() -> Self {
        State {
            index: 0usize,
            row: 1u64,
            col: 1u64,
        }
    }

    #[inline(always)]
    pub fn index(&self) -> usize { self.index }
    #[inline(always)]
    pub fn row(&self) -> u64 { self.row }
    #[inline(always)]
    pub fn col(&self) -> u64 { self.col }

    #[inline]
    pub(crate) fn read(&mut self, is_newline: bool) {
        if is_newline {
            self.row += 1;
            self.col = 1;
        } else if self.index != 0 {
            self.col += 1;
        }

        self.index += 1;
    }

    #[inline]
    pub(crate) fn update(&mut self, other: &State) {
        self.index = other.index;
        self.row = other.row;
        self.col = other.col;
    }
}
