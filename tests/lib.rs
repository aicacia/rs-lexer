extern crate lexer;
use lexer::Lexer;


#[test]
fn test_lexer() {
    let mut lexer = Lexer::new(String::from("(defn add_one [x] (+ x 1.0 \"asdf\"))"));

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

    test_next!("(", "operator");
    test_next!("defn", "name");
    test_next!("add_one", "name");
    test_next!("[", "operator");
    test_next!("x", "name");
    test_next!("]", "operator");
    test_next!("(", "operator");
    test_next!("+", "operator");
    test_next!("x", "name");
    test_next!("1.0", "number");
    test_next!("\"asdf\"", "string");
    test_next!(")", "operator");
    test_next!(")", "operator");
}
