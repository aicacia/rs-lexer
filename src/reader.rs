use super::{Input, State};


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReaderOption<T> {
    Some(T),
    Empty,
    None,
}


pub trait Reader<T>: Sync + Send {

    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(&self, &mut Input, &State, &mut State) -> ReaderOption<T>;
}
