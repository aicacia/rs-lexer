#![feature(test)]

extern crate test;

extern crate lisp;

use test::Bencher;

use lisp::{readers, Token};

#[bench]
fn bench_lexer(b: &mut Bencher) {
  let readers = readers();

  b.iter(|| {
    let lexer = readers.tokens("(hello,\n \"Hello, world!\",\n 10,\n true,\n false,\n:keyword)".chars());
    let _: Vec<Token> = lexer.map(Result::unwrap).collect();
  });
}
