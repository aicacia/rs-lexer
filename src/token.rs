use core::fmt::{self, Debug, Display};

use super::TokenMeta;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<T> {
    meta: TokenMeta,
    value: T,
}

unsafe impl<T> Send for Token<T>
where
    T: Send,
{
}
unsafe impl<T> Sync for Token<T>
where
    T: Sync,
{
}

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
    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> fmt::Debug for Token<T>
where
    T: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let meta = self.meta();

        f.debug_struct("Token")
            .field("value", &self.value)
            .field("index", &meta.index_start())
            .field("length", &meta.len())
            .field("lines", &meta.line_count())
            .field("row", &meta.row_start())
            .field("col", &meta.col_start())
            .finish()
    }
}

impl<T> fmt::Display for Token<T>
where
    T: Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let meta = self.meta();

        f.debug_struct("Token")
            .field("value", &format!("{}", self.value))
            .field("index", &meta.index_start())
            .field("length", &meta.len())
            .field("lines", &meta.line_count())
            .field("row", &meta.row_start())
            .field("col", &meta.col_start())
            .finish()
    }
}
