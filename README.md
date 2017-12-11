lexer
=====

plugin based lexical reader


```rust
extern crate lexer;


use lexer::{Token, Input, Reader, ReadersBuilder, State, TokenMeta};


#[derive(Debug)]
pub enum TokenKind {
    Whitespace,
    Identifier,
}

#[derive(Debug)]
pub enum TokenValue {
    Chr(char),
    Str(String),
}

pub type MyToken = Token<TokenKind, TokenValue>;


pub struct WhitespaceReader;

impl Reader<MyToken> for WhitespaceReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(&self, input: &mut Input, current: &State, next: &mut State) -> ReaderOption<MyToken> {
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
                    } else {
                        break;
                    }
                }

                ReaderOption::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Whitespace,
                    TokenValue::Str(string)
                ))
            } else {
                ReaderOption::None
            },
            None => ReaderOption::None,
        }
    }
}

pub struct IdentifierReader;

impl Reader<MyToken> for IdentifierReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        1usize
    }

    fn read(&self, input: &mut Input, current: &State, next: &mut State) -> ReaderOption<MyToken> {
        match input.read(next) {
            Some(ch) => if ch.is_alphabetic() {
                let mut string = String::new();

                string.push(ch);

                while !input.done(next) {
                    if let Some(ch) = input.peek(next, 0) {
                        if ch.is_alphanumeric() {
                            input.read(next);
                            string.push(ch);
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                ReaderOption::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Identifier,
                    TokenValue::Str(string)
                ))
            } else {
                ReaderOption::None
            },
            None => ReaderOption::None,
        }
    }
}


fn main() {
    let readers = ReadersBuilder::new()
        .add(WhitespaceReader)
        .add(IdentifierReader)
        .build();

    let vec: Vec<char> = "Hello world\n".chars().collect();
    let lexer = readers.lexer(vec);
    let tokens: Vec<MyToken> = lexer.collect();

    println!("{:#?}", tokens);
}
```
