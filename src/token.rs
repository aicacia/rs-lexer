use collections::string::String;

use core::fmt::{self, Debug};
use core::hash::Hash;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TokenMeta {
    index_start: u64,
    index_end: u64,
    col_start: u64,
    col_end: u64,
    line_start: u64,
    line_end: u64,
}

impl TokenMeta {
    #[inline]
    pub fn new(
        index_start: u64,
        index_end: u64,
        col_start: u64,
        col_end: u64,
        line_start: u64,
        line_end: u64
    ) -> Self {
        assert!(index_end > index_start, "token meta error: end index cannot be less than the start row of a token.");
        assert!(col_end >= col_start, "token meta error: end col cannot be less than the start col of a token.");
        assert!(line_end >= line_start, "token meta error: end line cannot be less than the start line of a token.");

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
    pub fn index_start(&self) -> u64 { self.index_start }
    #[inline(always)]
    pub fn index_end(&self) -> u64 { self.index_end }

    #[inline(always)]
    pub fn col_start(&self) -> u64 { self.col_start }
    #[inline(always)]
    pub fn col_end(&self) -> u64 { self.col_end }

    #[inline(always)]
    pub fn line_start(&self) -> u64 { self.line_start }
    #[inline(always)]
    pub fn line_end(&self) -> u64 { self.line_end }

    #[inline(always)]
    pub fn len(&self) -> u64 {
        self.index_end - self.index_start
    }

    #[inline(always)]
    pub fn line_count(&self) -> u64 {
        (self.line_end - self.line_start) + 1
    }

    #[inline(always)]
    pub fn col_count(&self) -> u64 {
        (self.col_end - self.col_start) + 1
    }
}


#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Token<T>
    where T: Clone + Eq + PartialEq + Hash
{
    kind: T,
    meta: TokenMeta,
    value: String,
}

impl<T> Token<T>
    where T: Clone + Eq + PartialEq + Hash
{

    #[inline(always)]
    pub fn new(meta: TokenMeta, kind: T, value: String) -> Self {
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
    pub fn value(&self) -> &String { &self.value }

    #[inline(always)]
    pub fn is_kind(&self, t: &T) -> bool {
        &self.kind == t
    }
}

impl<T> fmt::Debug for Token<T>
    where T: Debug + Clone + Eq + PartialEq + Hash
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", {
            let m = &self.meta;

            format!("<{:?} {:?} ls=({}) lc=({}) cols=({},{})>",
                self.kind,
                self.value,
                m.line_start,
                m.line_count(),
                m.col_start,
                m.col_end
            )
        })
    }
}
