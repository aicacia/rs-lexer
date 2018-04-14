use super::{Input, ReaderResult, State};

pub trait Reader<T, E, D = ()> {
    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(&self, &mut Input, &State, &mut State, &mut D) -> ReaderResult<T, E>;
}
