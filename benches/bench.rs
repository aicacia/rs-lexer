#![feature(test)]

extern crate test;

extern crate lexer;

use test::Bencher;

use lexer::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenValue {
  Whitespace(String),
  Identifier(String),
}

pub type MyToken = Token<TokenValue>;
pub type MyError = TokenError<&'static str>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<MyToken, MyError> for WhitespaceReader {
  #[inline(always)]
  fn priority(&self) -> usize {
    1usize
  }

  fn read(
    &self,
    _reader: &Readers<MyToken, MyError>,
    input: &mut dyn Input,
    current: &State,
    next: &mut State,
  ) -> ReaderResult<MyToken, MyError> {
    match input.read_whitespaces(next) {
      Some(string) => ReaderResult::Some(Token::new(
        TokenMeta::new_state_meta(current, next),
        TokenValue::Whitespace(string),
      )),
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<MyToken, MyError> for IdentifierReader {
  #[inline(always)]
  fn priority(&self) -> usize {
    0usize
  }

  fn read(
    &self,
    _reader: &Readers<MyToken, MyError>,
    input: &mut dyn Input,
    current: &State,
    next: &mut State,
  ) -> ReaderResult<MyToken, MyError> {
    match input.read(next) {
      Some(ch) => {
        if ch.is_alphabetic() {
          let mut string = String::new();

          string.push(ch);

          while let Some(ch) = input.peek(next, 0) {
            if ch.is_alphanumeric() {
              input.read(next);
              string.push(ch);
            } else {
              break;
            }
          }

          ReaderResult::Some(Token::new(
            TokenMeta::new_state_meta(current, next),
            TokenValue::Identifier(string),
          ))
        } else {
          ReaderResult::None
        }
      }
      None => ReaderResult::None,
    }
  }
}

#[bench]
fn bench_lexer(b: &mut Bencher) {
  let readers = ReadersBuilder::new()
    .add(WhitespaceReader)
    .add(IdentifierReader)
    .build();

  b.iter(|| {
    let lexer = readers.lexer(" def  \n\t   name ".chars());
    let _: Vec<MyToken> = lexer.map(|t| t.unwrap()).collect();
  });
}
