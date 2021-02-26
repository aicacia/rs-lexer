use super::TokenMeta;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
pub struct Token<T> {
  meta: TokenMeta,
  value: T,
}

unsafe impl<T> Send for Token<T> where T: Send {}
unsafe impl<T> Sync for Token<T> where T: Sync {}

impl<T> Token<T> {
  #[inline(always)]
  pub fn new(meta: TokenMeta, value: T) -> Self {
    Token {
      meta: meta,
      value: value,
    }
  }

  #[inline(always)]
  pub fn meta(&self) -> &TokenMeta {
    &self.meta
  }
  #[inline(always)]
  pub fn into_meta(self) -> TokenMeta {
    self.meta
  }
  #[inline(always)]
  pub fn value(&self) -> &T {
    &self.value
  }
  #[inline(always)]
  pub fn into_value(self) -> T {
    self.value
  }
}
