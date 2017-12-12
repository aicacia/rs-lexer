use super::{Input, State};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReaderResult<T, E> {
    Some(T),
    Err(E),
    Empty,
    None,
}


pub trait Reader<T, E> {

    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(&self, &mut Input, &State, &mut State) -> ReaderResult<T, E>;
}
