lexer
=====

plugin based lexical reader


```rust
extern crate lexer;


use lexer::{Token, TokenError, TokenMeta, Input, Reader, ReadersBuilder, State, ReaderResult};


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
pub type MyError = TokenError<&'static str>;


pub struct WhitespaceReader;

impl Reader<MyToken, MyError> for WhitespaceReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(&self, input: &mut Input, current: &State, next: &mut State) -> ReaderResult<MyToken, MyError> {
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

                ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Whitespace,
                    TokenValue::Str(string)
                ))
            } else {
                ReaderResult::None
            },
            None => ReaderResult::None,
        }
    }
}

pub struct IdentifierReader;

impl Reader<MyToken, MyError> for IdentifierReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        1usize
    }

    fn read(&self, input: &mut Input, current: &State, next: &mut State) -> ReaderResult<MyToken, MyError> {
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

                ReaderResult::Some(Token::new(
                    TokenMeta::new_state_meta(current, next),
                    TokenKind::Identifier,
                    TokenValue::Str(string)
                ))
            } else {
                ReaderResult::None
            },
            None => ReaderResult::None,
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
    let tokens: Vec<MyToken> = lexer.map(|t| t.unwrap()).collect();

    println!("{:#?}", tokens);
}
```
