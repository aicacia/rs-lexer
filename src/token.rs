use core::fmt::{self, Debug, Display};

use super::TokenMeta;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<T, V> {
    meta: TokenMeta,
    kind: T,
    value: V,
}

unsafe impl<T, V> Send for Token<T, V>
where
    T: Send,
    V: Send,
{
}
unsafe impl<T, V> Sync for Token<T, V>
where
    T: Sync,
    V: Sync,
{
}

impl<T, V> Token<T, V> {
    #[inline(always)]
    pub fn new(meta: TokenMeta, kind: T, value: V) -> Self {
        Token {
            meta: meta,
            kind: kind,
            value: value,
        }
    }

    #[inline(always)]
    pub fn meta(&self) -> &TokenMeta {
        &self.meta
    }
    #[inline(always)]
    pub fn kind(&self) -> &T {
        &self.kind
    }
    #[inline(always)]
    pub fn value(&self) -> &V {
        &self.value
    }
}

impl<T, V> fmt::Debug for Token<T, V>
where
    T: Debug,
    V: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let meta = self.meta();

        f.debug_struct("Token")
            .field("value", &self.value)
            .field("kind", &self.kind)
            .field("index", &meta.index_start())
            .field("length", &meta.len())
            .field("lines", &meta.line_count())
            .field("row", &meta.row_start())
            .field("col", &meta.col_start())
            .finish()
    }
}

impl<T, V> fmt::Display for Token<T, V>
where
    T: Display,
    V: Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let meta = self.meta();

        f.debug_struct("Token")
            .field("value", &format!("{}", self.value))
            .field("kind", &format!("{}", self.kind))
            .field("index", &meta.index_start())
            .field("length", &meta.len())
            .field("lines", &meta.line_count())
            .field("row", &meta.row_start())
            .field("col", &meta.col_start())
            .finish()
    }
}
