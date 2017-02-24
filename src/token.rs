use collections::string::String;

use super::kind::Kind;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Token {
    kind: Kind,
    value: String,
    index: usize,
    row: usize,
    column: usize,
}

impl Token {
    #[inline(always)]
    pub fn new(value: String, kind: Kind, index: usize, row: usize, column: usize) -> Token {
        Token {
            kind: kind,
            value: value,
            index: index,
            row: row,
            column: column,
        }
    }
    #[inline(always)]
    pub fn kind(&self) -> Kind {
        self.kind
    }
    #[inline(always)]
    pub fn value(&self) -> &String {
        &self.value
    }
    #[inline(always)]
    pub fn index(&self) -> usize {
        self.index
    }
    #[inline(always)]
    pub fn row(&self) -> usize {
        self.row
    }
    #[inline(always)]
    pub fn column(&self) -> usize {
        self.column
    }
}
