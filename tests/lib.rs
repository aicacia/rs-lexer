extern crate lexer;
use lexer::Lexer;


#[test]
fn test_lexer_iter() {
    let mut lexer = Lexer::new(String::from("(defn add [x y] (+ x y))"));

    macro_rules! test_next {
        ($x:expr) => (
            match lexer.next() {
                Some(token) => assert_eq!(token.value, $x),
                None => panic!("should not fail"),
            }
        );
    }

    test_next!("(");
    test_next!("defn");
    test_next!("add");
    test_next!("[");
    test_next!("x");
    test_next!("y");
    test_next!("]");
    test_next!("(");
    test_next!("+");
    test_next!("x");
    test_next!("y");
    test_next!(")");
    test_next!(")");
}
