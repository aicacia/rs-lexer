extern crate lexer;
extern crate lisp;

use lisp::{readers, Token, TokenValue};

#[test]
fn test_lexer_identifier() {
  use std::fs::File;
  use std::io::Read;

  let readers = readers();

  let string = {
    let mut buf = Vec::new();
    File::open("tests/file.txt")
      .unwrap()
      .read_to_end(&mut buf)
      .unwrap();
    String::from_utf8(buf).unwrap()
  };

  println!("{}", string);
  let lexer = readers.lexer(string.chars());
  let tokens: Vec<Token> = lexer.map(Result::unwrap).collect();

  assert_eq!(tokens.len(), 1);

  if let Some(&TokenValue::List(ref tokens)) = tokens.get(0).map(Token::value) {
    let first = tokens.first().unwrap();
    assert_eq!(first.meta().col_start(), 1);
    assert_eq!(first.meta().col_end(), 7);
    assert_eq!(first.meta().col_count(), 6);
    assert_eq!(first.meta().line_start(), 1);
    assert_eq!(first.meta().line_end(), 1);
    assert_eq!(first.meta().line_count(), 0);
    assert_eq!(first.meta().len(), 6);
  }
}
