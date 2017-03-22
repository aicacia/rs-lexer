use std::fmt::{self, Debug};
use std::hash::Hash;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TokenMeta {
    col_start: u64,
    col_end: u64,
    line_start: u64,
    line_end: u64,
}

impl TokenMeta {

    pub fn new(
        col_start: u64,
        col_end: u64,
        line_start: u64,
        line_end: u64
    ) -> Self {
        if line_end < line_start {
            panic!("meta error: start row cannot be less than the end row of a token.");
        }

        TokenMeta {
            col_start: col_start,
            col_end: col_end,
            line_start: line_start,
            line_end: line_end,
        }
    }

    #[inline(always)]
    pub fn col_start(&self) -> u64 { self.col_start }
    #[inline(always)]
    pub fn col_end(&self) -> u64 { self.col_end }
    #[inline(always)]
    pub fn line_start(&self) -> u64 { self.line_start }
    #[inline(always)]
    pub fn line_end(&self) -> u64 { self.line_end }

    #[inline]
    pub fn line_count(&self) -> u64 {
        (self.line_end - self.line_start) + 1
    }

    #[inline]
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

    #[inline]
    pub fn new(meta: TokenMeta, kind: T, val: String) -> Self {
        Token {
            meta: meta,
            kind: kind,
            value: val,
        }
    }

    #[inline(always)]
    pub fn meta(&self) -> &TokenMeta { &self.meta }
    #[inline(always)]
    pub fn kind(&self) -> &T { &self.kind }
    #[inline(always)]
    pub fn value(&self) -> &String { &self.value }

    #[inline]
    pub fn is_kind(&self, t: &T) -> bool {
        &self.kind == t
    }
}

impl<T> fmt::Display for Token<T> 
    where T: Debug + Clone + Eq + PartialEq + Hash
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", {
            let m = &self.meta;

            format!("<{:?} ls=({}) lc=({}) cols=({},{})>",
                self.kind,
                m.line_start,
                m.line_count(),
                m.col_start,
                m.col_end
            )
        })
    }
}
