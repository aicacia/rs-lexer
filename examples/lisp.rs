extern crate lexer;
extern crate lisp;

use lisp::readers;

fn main() {
  let readers = readers();

  let mut tokens =
    readers.tokens("(hello,\n \"Hello, world!\",\n 10,\n true,\n false,\n:keyword)".chars());

  let token = tokens
    .next()
    .map(Result::unwrap)
    .map(lexer::Token::into_value)
    .unwrap();

  println!("{}", token);
}
