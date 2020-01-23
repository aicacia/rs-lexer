# lexer

plugin based lexical reader

```rust
extern crate lexer;

use std::collections::LinkedList;
use std::fmt::{self, Write};

use lexer::{Input, Reader, ReaderResult, Readers, ReadersBuilder, State, TokenMeta};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenValue {
  Number(isize),
  String(String),
  Keyword(String),
  Identifier(String),
  List(LinkedList<Token>),
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
        if ch.is_whitespace() || ch == ',' {
          while let Some(ch) = input.peek(next, 0) {
            if ch.is_whitespace() || ch == ',' {
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
        if ch.is_numeric() {
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
            if ch.is_alphanumeric() {
              input.read(next);
              string.push(ch);
            } else {
              break;
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
          let mut list = LinkedList::new();

          while let Some(ch) = input.peek(next, 0) {
            if ch == ')' {
              input.read(next);
              break;
            } else {
              match lexer::next(readers, input, next) {
                Some(Ok(token)) => {
                  list.push_back(token);
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

fn main() {
  let readers = ReadersBuilder::new()
    .add(WhitespaceReader)
    .add(NumberReader)
    .add(StringReader)
    .add(KeywordReader)
    .add(IdentifierReader)
    .add(ListReader)
    .build();

  let lexer =
    readers.lexer("(hello,\n \"Hello, world!\",\n 10,\n true,\n false,\n:keyword)".chars());
  let tokens: Vec<Token> = lexer.map(Result::unwrap).collect();
  let token = tokens.get(0).map(lexer::Token::value).unwrap();

  println!("{:#?}", tokens);
}
```
