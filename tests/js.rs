use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate lexer;
use self::lexer::Lexer;


#[test]
fn test_lexer() {
    let path = Path::new(file!()).parent().unwrap().join(Path::new("example.js"));

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), Error::description(&why)),
        Ok(file) => file,
    };

    let mut string = String::new();
    match file.read_to_string(&mut string) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), Error::description(&why)),
        _ => (),
    }

    let mut lexer = Lexer::new(string, String::from("[](){}<>?,.:;/~!@#%^&*-+="));

    macro_rules! test_next {
        ($value:expr, $kind:expr) => (
            match lexer.next() {
                Some(token) => {
                    assert_eq!(token.value, $value);
                    assert_eq!(token.kind, $kind);
                },
                None => panic!("should not fail"),
            }
        );
    }

    test_next!("function", "symbol");
    test_next!("fac", "symbol");
    test_next!("(", "syntex");
    test_next!("x", "symbol");
    test_next!(")", "syntex");
    test_next!("{", "syntex");
    test_next!("if", "symbol");
    test_next!("(", "syntex");
    test_next!("x", "symbol");
    test_next!("<", "syntex");
    test_next!("=", "syntex");
    test_next!("0x01", "number");
    test_next!(")", "syntex");
    test_next!("{", "syntex");
    test_next!("return", "symbol");
    test_next!("1.0", "number");
    test_next!(";", "syntex");
    test_next!("}", "syntex");
    test_next!("else", "symbol");
    test_next!("{", "syntex");
    test_next!("return", "symbol");
    test_next!("x", "symbol");
    test_next!("*", "syntex");
    test_next!("fac", "symbol");
    test_next!("(", "syntex");
    test_next!("x", "symbol");
    test_next!("-", "syntex");
    test_next!("1", "number");
    test_next!(")", "syntex");
    test_next!(";", "syntex");
    test_next!("}", "syntex");
    test_next!("}", "syntex");
}
