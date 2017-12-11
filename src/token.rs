use core::fmt::{self, Debug, Display};

use super::TokenMeta;


#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Token<T, V> {
    meta: TokenMeta,
    kind: T,
    value: V,
}

unsafe impl<T, V> Send for Token<T, V>
    where T: Send,
          V: Send,
{}
unsafe impl<T, V> Sync for Token<T, V>
    where T: Sync,
          V: Sync,
{}

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
    pub fn meta(&self) -> &TokenMeta { &self.meta }
    #[inline(always)]
    pub fn kind(&self) -> &T { &self.kind }
    #[inline(always)]
    pub fn value(&self) -> &V { &self.value }
}

impl<T, V> fmt::Debug for Token<T, V>
    where T: Debug,
          V: Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let meta = self.meta();

        write!(f, "{:?} {{ index: {}, length: {}, lines: {}, row: {}, col: {} }}",
            self.value,
            meta.index_start(),
            meta.len(),
            meta.line_count(),
            meta.row_start(),
            meta.col_start()
        )
    }
}

impl<T, V> fmt::Display for Token<T, V>
    where T: Display,
          V: Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let meta = self.meta();

        write!(f, "{} {{ index: {}, length: {}, lines: {}, row: {}, col: {} }}",
            self.value,
            meta.index_start(),
            meta.len(),
            meta.line_count(),
            meta.row_start(),
            meta.col_start(),
        )
    }
}
