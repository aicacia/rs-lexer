#![feature(test)]

extern crate test;

extern crate lisp;

use test::Bencher;

use lisp::{readers, Token};

#[bench]
fn bench_lexer(b: &mut Bencher) {
  let readers = readers();

  b.iter(|| {
    let lexer = readers.tokens("(def  \n\t   name)".chars());
    let _: Vec<Token> = lexer.map(Result::unwrap).collect();
  });
}
