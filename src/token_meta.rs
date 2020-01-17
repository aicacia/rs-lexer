use core::fmt;

use super::State;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TokenMeta {
  index_start: u64,
  index_end: u64,
  col_start: u64,
  col_end: u64,
  line_start: u64,
  line_end: u64,
}

unsafe impl Send for TokenMeta {}
unsafe impl Sync for TokenMeta {}

impl TokenMeta {
  #[inline]
  pub fn new(
    index_start: u64,
    index_end: u64,
    col_start: u64,
    col_end: u64,
    line_start: u64,
    line_end: u64,
  ) -> Self {
    debug_assert!(
      index_end > index_start,
      "token meta error: end index cannot be less than the start row of a token."
    );
    debug_assert!(
      line_end >= line_start,
      "token meta error: end line cannot be less than the start line of a token."
    );

    TokenMeta {
      index_start: index_start,
      index_end: index_end,
      col_start: col_start,
      col_end: col_end,
      line_start: line_start,
      line_end: line_end,
    }
  }

  #[inline(always)]
  pub fn new_state_meta<'a>(current_state: &'a State, state: &'a State) -> TokenMeta {
    TokenMeta::new(
      current_state.index() as u64,
      state.index() as u64,
      current_state.col(),
      state.col(),
      current_state.row(),
      state.row(),
    )
  }

  #[inline(always)]
  pub fn index_start(&self) -> u64 {
    self.index_start
  }
  #[inline(always)]
  pub fn index_end(&self) -> u64 {
    self.index_end
  }

  #[inline(always)]
  pub fn row_start(&self) -> u64 {
    self.line_start
  }
  #[inline(always)]
  pub fn row_end(&self) -> u64 {
    self.line_end
  }

  #[inline(always)]
  pub fn col_start(&self) -> u64 {
    self.col_start
  }
  #[inline(always)]
  pub fn col_end(&self) -> u64 {
    self.col_end
  }

  #[inline(always)]
  pub fn line_start(&self) -> u64 {
    self.line_start
  }
  #[inline(always)]
  pub fn line_end(&self) -> u64 {
    self.line_end
  }

  #[inline(always)]
  pub fn len(&self) -> u64 {
    self.index_end - self.index_start
  }

  #[inline(always)]
  pub fn line_count(&self) -> u64 {
    self.line_end - self.line_start
  }

  #[inline(always)]
  pub fn col_count(&self) -> u64 {
    self.col_end - self.col_start
  }
}

impl fmt::Display for TokenMeta {
  #[inline(always)]
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    fmt::Debug::fmt(self, f)
  }
}
