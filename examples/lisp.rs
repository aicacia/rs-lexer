extern crate lisp;

use lisp::{readers, Token};

fn main() {
  let readers = readers();

  let lexer =
    readers.lexer("(hello,\n \"Hello, world!\",\n 10,\n true,\n false,\n:keyword)".chars());
  let tokens: Vec<Token> = lexer.map(Result::unwrap).collect();
  let token = tokens.get(0).map(lexer::Token::value).unwrap();

  println!("{:#?}", tokens);
  println!("{}", token);
}
