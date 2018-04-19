use super::{Input, ReaderResult, Readers, State};

pub trait Reader<T, E> {
    #[inline(always)]
    fn priority(&self) -> usize {
        0
    }

    fn read(&self, &Readers<T, E>, &mut Input, &State, &mut State) -> ReaderResult<T, E>;
}
