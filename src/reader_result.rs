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
