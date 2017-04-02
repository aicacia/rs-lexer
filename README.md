lexer [![Build Status](https://travis-ci.org/nathanfaucett/rs-lexer.svg?branch=master)](https://travis-ci.org/nathanfaucett/rs-lexer)
=====

plugin based lexical reader


```rust
extern crate lexer;


use lexer::*;


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenKind {
    WHITESPACE,
    IDENTIFIER,
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<TokenKind> for WhitespaceReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        1usize
    }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token<TokenKind>> {
        let ch = input.read(next);

        if ch.is_whitespace() {
            let mut string = String::new();

            string.push(ch);

            while !input.done(next) {
                let ch = input.peek(next, 0);

                if ch.is_whitespace() {
                    input.read(next);
                    string.push(ch);
                } else {
                    break;
                }
            }

            Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                TokenKind::WHITESPACE,
                string
            ))
        } else {
            None
        }
    }
}

fn main() {
    let mut lexer = Lexer::<TokenKind, Vec<char>>::from("   \n\t   ");

    lexer.readers
        .add(WhitespaceReader)
        .sort();

    let tokens: Vec<Token<TokenKind>> = lexer.collect();
    println!("{:?}", tokens);
}
```
