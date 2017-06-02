lexer [![Build Status](https://travis-ci.org/nathanfaucett/rs-lexer.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-lexer)
=====

plugin based lexical reader


```rust
extern crate lexer;


use lexer::{Lexer, Input, State, TokenMeta, Reader};


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenKind {
    WHITESPACE,
    IDENTIFIER,
}

pub type TokenValue = String;

pub type Token = lexer::Token<TokenKind, TokenValue>;


#[derive(Clone, Eq, PartialEq)]
pub struct WhitespaceReader;

impl Reader<Token> for WhitespaceReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        1usize
    }

    #[inline]
    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token> {
        match input.read(next) {
            Some(ch) => if ch.is_whitespace() {
                let mut string = String::new();

                string.push(ch);

                while !input.done(next) {
                    if let Some(ch) = input.peek(next, 0) {
                      if ch.is_whitespace() {
                          input.read(next);
                          string.push(ch);
                      } else {
                          break;
                      }
                    }
                }

                Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::WHITESPACE,
                    string
                ))
            } else {
                None
            },
            None => None
        }
    }
}

fn main() {
    let mut lexer = Lexer::from("   \n\t   ");

    lexer.readers
        .add(WhitespaceReader)
        .sort();

    let tokens: Vec<Token> = lexer.collect();
    println!("{:?}", tokens);
}
```
