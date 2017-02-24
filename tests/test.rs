extern crate lexer;


use lexer::{Lexer, Kind};


macro_rules! token_eq {
    ($lexer: ident, $value: expr, $kind: expr) => {
        let token = $lexer.next().unwrap();
        assert_eq!(token.value(), $value);
        assert_eq!(token.kind(), $kind);
    };
}

#[test]
fn test_lexer() {
    let mut lexer = Lexer::new("
        symbol
        \"double quoted\"
        'char'
        10.0
        0xff
        128
        5usize
        {}
        ()
    ");

    token_eq!(lexer, "symbol", Kind::Sym);
    token_eq!(lexer, "double quoted", Kind::Str);
    token_eq!(lexer, "char", Kind::Chr);
    token_eq!(lexer, "10.0", Kind::Num);
    token_eq!(lexer, "0xff", Kind::Num);
    token_eq!(lexer, "128", Kind::Num);
    token_eq!(lexer, "5", Kind::Num);
    token_eq!(lexer, "usize", Kind::Sym);
    token_eq!(lexer, "{", Kind::Syn);
    token_eq!(lexer, "}", Kind::Syn);
    token_eq!(lexer, "(", Kind::Syn);
    token_eq!(lexer, ")", Kind::Syn);
}

#[test]
fn test_lang() {
    let mut lexer = Lexer::new("
        pub struct Type<T> {
            value: T,
        }
    ");
    token_eq!(lexer, "pub", Kind::Sym);
    token_eq!(lexer, "struct", Kind::Sym);
    token_eq!(lexer, "Type", Kind::Sym);
    token_eq!(lexer, "<", Kind::Syn);
    token_eq!(lexer, "T", Kind::Sym);
    token_eq!(lexer, ">", Kind::Syn);
    token_eq!(lexer, "{", Kind::Syn);
    token_eq!(lexer, "value", Kind::Sym);
    token_eq!(lexer, ":", Kind::Syn);
    token_eq!(lexer, "T", Kind::Sym);
    token_eq!(lexer, "}", Kind::Syn);
}

#[test]
fn test_value_and_kind() {
    let mut lexer = Lexer::new("(add ... 1 2)");

    token_eq!(lexer, "(", Kind::Syn);
    token_eq!(lexer, "add", Kind::Sym);
    token_eq!(lexer, ".", Kind::Syn);
    token_eq!(lexer, ".", Kind::Syn);
    token_eq!(lexer, ".", Kind::Syn);
    token_eq!(lexer, "1", Kind::Num);
    token_eq!(lexer, "2", Kind::Num);
    token_eq!(lexer, ")", Kind::Syn);
}
