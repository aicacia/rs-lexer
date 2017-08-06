use core::fmt::{self, Debug, Display};
use core::hash::Hash;

use super::state::State;


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
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
        line_end: u64
    ) -> Self {
        assert!(
            index_end > index_start,
            "token meta error: end index cannot be less than the start row of a token."
        );
        assert!(
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
            state.row()
        )
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
pub struct Token<T, V>
    where T: Clone + Eq + PartialEq + Hash
{
    meta: TokenMeta,
    kind: T,
    value: V,
}

unsafe impl<T: Send + Clone + Eq + PartialEq + Hash, V: Send> Send for Token<T, V> {}
unsafe impl<T: Sync + Clone + Eq + PartialEq + Hash, V: Sync> Sync for Token<T, V> {}

impl<T, V> Token<T, V>
    where T: Clone + Eq + PartialEq + Hash
{

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

    #[inline(always)]
    pub fn is_kind(&self, t: &T) -> bool {
        &self.kind == t
    }
}

impl<T, V> fmt::Debug for Token<T, V>
    where T: Debug + Clone + Eq + PartialEq + Hash,
          V: Debug
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", {
            let meta = self.meta();

            format!("<{:?} {:?} line_start: {}, line_count: {}, row: {}, col: {}>",
                self.kind,
                self.value,
                meta.line_start,
                meta.line_count(),
                meta.col_start,
                meta.col_end
            )
        })
    }
}

impl<T, V> fmt::Display for Token<T, V>
    where T: Display + Clone + Eq + PartialEq + Hash,
          V: Display
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", {
            let meta = self.meta();

            format!("<{} {} line_start: {}, line_count: {}, row: {}, col: {}>",
                self.kind,
                self.value,
                meta.line_start,
                meta.line_count(),
                meta.col_start,
                meta.col_end
            )
        })
    }
}
