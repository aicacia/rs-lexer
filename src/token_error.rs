use super::TokenMeta;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Debug, Ord, Hash)]
pub struct TokenError<E> {
  meta: TokenMeta,
  error: E,
}

unsafe impl<E> Send for TokenError<E> where E: Send {}
unsafe impl<E> Sync for TokenError<E> where E: Sync {}

impl<E> TokenError<E> {
  #[inline(always)]
  pub fn new(meta: TokenMeta, error: E) -> Self {
    TokenError {
      meta: meta,
      error: error,
    }
  }

  #[inline(always)]
  pub fn meta(&self) -> &TokenMeta {
    &self.meta
  }
  #[inline(always)]
  pub fn error(&self) -> &E {
    &self.error
  }
}
