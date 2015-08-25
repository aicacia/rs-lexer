#[derive(Debug)]
pub struct Token {
    pub value: String,
    pub start: usize,
    pub end: usize,
}

impl Token {
    #[inline(always)]
    pub fn new(value: String, start: usize, end: usize) -> Token {
        Token {
            value: value,
            start: start,
            end: end,
        }
    }
}
