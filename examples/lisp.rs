#![feature(io)]

extern crate lexer;

use std::collections::LinkedList;

use lexer::{Input, Reader, ReaderResult, Readers, ReadersBuilder, State, TokenMeta};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenValue {
    Number(isize),
    String(String),
    Identifier(String),
    List(LinkedList<Token>),
}

pub type Token = lexer::Token<TokenValue>;
pub type TokenError = lexer::TokenError<&'static str>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<Token, TokenError> for WhitespaceReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(
        &self,
        _: &Readers<Token, TokenError>,
        input: &mut Input,
        _: &State,
        next: &mut State,
    ) -> ReaderResult<Token, TokenError> {
        match input.read(next) {
            Some(ch) => if ch.is_whitespace() || ch == ',' {
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
            },
            None => ReaderResult::None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct NumberReader;

impl Reader<Token, TokenError> for NumberReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        1usize
    }

    fn read(
        &self,
        _: &Readers<Token, TokenError>,
        input: &mut Input,
        current: &State,
        next: &mut State,
    ) -> ReaderResult<Token, TokenError> {
        match input.read(next) {
            Some(ch) => if ch.is_numeric() {
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
            },
            None => ReaderResult::None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct StringReader;

impl Reader<Token, TokenError> for StringReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        2usize
    }

    fn read(
        &self,
        _: &Readers<Token, TokenError>,
        input: &mut Input,
        current: &State,
        next: &mut State,
    ) -> ReaderResult<Token, TokenError> {
        match input.read(next) {
            Some(ch) => if ch == '"' {
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
            },
            None => ReaderResult::None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<Token, TokenError> for IdentifierReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        3usize
    }

    fn read(
        &self,
        _: &Readers<Token, TokenError>,
        input: &mut Input,
        current: &State,
        next: &mut State,
    ) -> ReaderResult<Token, TokenError> {
        match input.read(next) {
            Some(ch) => if ch.is_alphabetic() {
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
            },
            None => ReaderResult::None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct ListReader;

impl Reader<Token, TokenError> for ListReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        4usize
    }

    fn read(
        &self,
        readers: &Readers<Token, TokenError>,
        input: &mut Input,
        current: &State,
        next: &mut State,
    ) -> ReaderResult<Token, TokenError> {
        match input.read(next) {
            Some(ch) => if ch == '(' {
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
            },
            None => ReaderResult::None,
        }
    }
}

fn main() {
    let readers = ReadersBuilder::new()
        .add(WhitespaceReader)
        .add(NumberReader)
        .add(StringReader)
        .add(IdentifierReader)
        .add(ListReader)
        .build();

    let lexer = readers.lexer("(hello,\n \"Hello, world!\",\n 10,\n true,\n false)".chars());
    let tokens: Vec<Token> = lexer.map(|t| t.unwrap()).collect();

    println!("{:#?}", tokens);
}
