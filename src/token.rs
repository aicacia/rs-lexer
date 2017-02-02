use collections::string::String;

use super::kind::Kind;


#[derive(Debug)]
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

    pub fn kind(&self) -> Kind { self.kind }
    pub fn value(&self) -> &String { &self.value }
    pub fn index(&self) -> usize { self.index }
    pub fn row(&self) -> usize { self.row }
    pub fn column(&self) -> usize { self.column }
}
