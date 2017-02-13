

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Kind {
    Str,
    Char,
    Symbol,
    Syntax,
    Number,
}
