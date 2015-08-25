#[derive(Debug)]
pub struct Token {
    pub kind: String,
    pub value: String,
    pub start: usize,
    pub end: usize,
}

impl Token {
    #[inline(always)]
    pub fn new(value: String, kind: String, start: usize, end: usize) -> Token {
        Token {
            kind: kind,
            value: value,
            start: start,
            end: end,
        }
    }
}
