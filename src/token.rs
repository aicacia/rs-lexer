use collections::string::String;


#[derive(Debug)]
pub struct Token {
    pub kind: String,
    pub value: String,
    pub row: usize,
    pub column: usize,
}

impl Token {
    #[inline(always)]
    pub fn new(value: String, kind: String, row: usize, column: usize) -> Token {
        Token {
            kind: kind,
            value: value,
            row: row,
            column: column,
        }
    }
}
