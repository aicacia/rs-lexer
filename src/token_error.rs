use core::fmt::{self, Debug, Display};

use super::TokenMeta;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<E> fmt::Debug for TokenError<E>
where
  E: Debug,
{
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let meta = self.meta();

    f.debug_struct("TokenError")
      .field("error", &self.error)
      .field("index", &meta.index_start())
      .field("length", &meta.len())
      .field("lines", &meta.line_count())
      .field("row", &meta.row_start())
      .field("col", &meta.col_start())
      .finish()
  }
}

impl<E> fmt::Display for TokenError<E>
where
  E: Display,
{
  #[inline]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let meta = self.meta();

    f.debug_struct("TokenError")
      .field("error", &format!("{}", self.error))
      .field("index", &meta.index_start())
      .field("length", &meta.len())
      .field("lines", &meta.line_count())
      .field("row", &meta.row_start())
      .field("col", &meta.col_start())
      .finish()
  }
}
