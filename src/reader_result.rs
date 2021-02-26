#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReaderResult<T, E> {
  Some(T),
  Err(E),
  Empty,
  None,
}

unsafe impl<T, E> Send for ReaderResult<T, E>
where
  T: Send,
  E: Send,
{
}
unsafe impl<T, E> Sync for ReaderResult<T, E>
where
  T: Send,
  E: Send,
{
}

impl<T, E> From<Result<T, E>> for ReaderResult<T, E> {
  #[inline]
  fn from(result: Result<T, E>) -> Self {
    match result {
      Ok(value) => ReaderResult::Some(value),
      Err(error) => ReaderResult::Err(error),
    }
  }
}

impl<T, E> From<Option<T>> for ReaderResult<T, E> {
  #[inline]
  fn from(option: Option<T>) -> Self {
    match option {
      Some(value) => ReaderResult::Some(value),
      None => ReaderResult::None,
    }
  }
}
