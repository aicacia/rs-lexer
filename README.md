# lexer

[![license](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue")](LICENSE-MIT)
[![API](https://docs.rs/lexer/badge.svg)](https://docs.rs/lexer)
[![Crate](https://img.shields.io/crates/v/lexer.svg)](https://crates.io/crates/lexer)
[![Test Status](https://github.com/aicacia/rs-lexer/workflows/Tests/badge.svg?event=push)](https://github.com/aicacia/rs-lexer/actions)

plugin based lexical reader

```rust
extern crate lexer;

use std::fmt::{self, Write};

use lexer::{Input, Reader, ReaderResult, Readers, ReadersBuilder, State, TokenMeta};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenValue {
  Number(isize),
  String(String),
  Keyword(String),
  Identifier(String),
  List(Vec<Token>),
}

impl fmt::Display for TokenValue {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      &TokenValue::Number(ref n) => write!(f, "{}", n),
      &TokenValue::String(ref s) => write!(f, "{:?}", s),
      &TokenValue::Keyword(ref s) => write!(f, ":{}", s),
      &TokenValue::Identifier(ref s) => write!(f, "{}", s),
      &TokenValue::List(ref list) => {
        f.write_char('(')?;

        let mut index = 0;

        for token in list {
          write!(f, "{}", token.value())?;

          index += 1;
          if index < list.len() {
            f.write_str(", ")?;
          }
        }

        f.write_char(')')
      }
    }
  }
}

pub type Token = lexer::Token<TokenValue>;
pub type TokenError = lexer::TokenError<&'static str>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<Token, TokenError> for WhitespaceReader {
  fn read(
    &self,
    _: &Readers<Token, TokenError>,
    input: &mut dyn Input,
    _: &State,
    next: &mut State,
  ) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if is_whitespace(ch) {
          while let Some(ch) = input.peek(next, 0) {
            if is_whitespace(ch) {
              input.read(next);
            } else {
              break;
            }
          }

          ReaderResult::Empty
        } else {
          ReaderResult::None
        }
      }
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NumberReader;

impl Reader<Token, TokenError> for NumberReader {
  fn read(
    &self,
    _: &Readers<Token, TokenError>,
    input: &mut dyn Input,
    current: &State,
    next: &mut State,
  ) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch.is_numeric() || ch == '-' {
          let mut string = String::new();

          string.push(ch);

          while let Some(ch) = input.peek(next, 0) {
            if ch.is_numeric() || ch == '_' {
              input.read(next);
              string.push(ch);
            } else {
              break;
            }
          }

          ReaderResult::Some(Token::new(
            TokenMeta::new_state_meta(current, next),
            TokenValue::Number(string.parse().unwrap()),
          ))
        } else {
          ReaderResult::None
        }
      }
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StringReader;

impl Reader<Token, TokenError> for StringReader {
  fn read(
    &self,
    _: &Readers<Token, TokenError>,
    input: &mut dyn Input,
    current: &State,
    next: &mut State,
  ) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch == '"' {
          let mut string = String::new();

          while let Some(ch) = input.read(next) {
            if ch == '"' {
              break;
            } else {
              string.push(ch);
            }
          }

          ReaderResult::Some(Token::new(
            TokenMeta::new_state_meta(current, next),
            TokenValue::String(string),
          ))
        } else {
          ReaderResult::None
        }
      }
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeywordReader;

impl Reader<Token, TokenError> for KeywordReader {
  fn read(
    &self,
    _: &Readers<Token, TokenError>,
    input: &mut dyn Input,
    current: &State,
    next: &mut State,
  ) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch == ':' {
          let mut string = String::new();

          while let Some(ch) = input.peek(next, 0) {
            if is_closer(ch) || is_whitespace(ch) {
              break;
            } else {
              input.read(next);
              string.push(ch);
            }
          }

          ReaderResult::Some(Token::new(
            TokenMeta::new_state_meta(current, next),
            TokenValue::Keyword(string),
          ))
        } else {
          ReaderResult::None
        }
      }
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ListReader;

impl Reader<Token, TokenError> for ListReader {
  fn read(
    &self,
    readers: &Readers<Token, TokenError>,
    input: &mut dyn Input,
    current: &State,
    next: &mut State,
  ) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        if ch == '(' {
          let mut list = Vec::new();

          while let Some(ch) = input.peek(next, 0) {
            if ch == ')' {
              input.read(next);
              break;
            } else {
              match lexer::read(readers, input, next) {
                Some(Ok(token)) => {
                  list.push(token);
                }
                Some(Err(error)) => {
                  return ReaderResult::Err(error);
                }
                _ => {
                  break;
                }
              }
            }
          }

          ReaderResult::Some(Token::new(
            TokenMeta::new_state_meta(current, next),
            TokenValue::List(list),
          ))
        } else {
          ReaderResult::None
        }
      }
      None => ReaderResult::None,
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<Token, TokenError> for IdentifierReader {
  fn read(
    &self,
    _: &Readers<Token, TokenError>,
    input: &mut dyn Input,
    current: &State,
    next: &mut State,
  ) -> ReaderResult<Token, TokenError> {
    match input.read(next) {
      Some(ch) => {
        let mut string = String::new();

        string.push(ch);

        while let Some(ch) = input.peek(next, 0) {
          if is_closer(ch) || is_whitespace(ch) {
            break;
          } else {
            input.read(next);
            string.push(ch);
          }
        }

        ReaderResult::Some(Token::new(
          TokenMeta::new_state_meta(current, next),
          TokenValue::Identifier(string),
        ))
      }
      None => ReaderResult::None,
    }
  }
}

#[inline]
fn is_whitespace(ch: char) -> bool {
  ch.is_whitespace() || ch == ','
}

fn is_closer(ch: char) -> bool {
  ch == ')'
}

pub fn readers() -> lexer::Readers<Token, TokenError> {
  ReadersBuilder::new()
    .add(WhitespaceReader)
    .add(NumberReader)
    .add(StringReader)
    .add(KeywordReader)
    .add(ListReader)
    .add(IdentifierReader)
    .build()
}

fn main() {
  let readers = readers();

  let string = "(def-fn hello () (println :Hello, \"World!\"))";

  let tokens = readers.read(string.chars());
  let tokens: Vec<Token> = tokens.map(Result::unwrap).collect();

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
```
