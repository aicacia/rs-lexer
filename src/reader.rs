use super::{Input, ReaderResult, Readers, State};

pub trait Reader<T, E> {
  fn read(
    &self,
    reader: &Readers<T, E>,
    input: &mut dyn Input,
    current: &State,
    next: &mut State,
  ) -> ReaderResult<T, E>;
}
