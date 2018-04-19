#![feature(io)]

extern crate lexer;

use lexer::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenValue {
    EmptyLines(usize),
    Whitespace(String),
    Identifier(String),
}

pub type MyToken = Token<TokenValue>;
pub type MyError = TokenError<&'static str>;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct EmptyLineReader;

impl Reader<MyToken, MyError> for EmptyLineReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(
        &self,
        _: &Readers<MyToken, MyError>,
        input: &mut Input,
        current: &State,
        next: &mut State,
    ) -> ReaderResult<MyToken, MyError> {
        let mut count = 0;

        {
            let mut lines = input.lines(next);

            while let Some(line) = lines.peek_line() {
                if line.is_empty() {
                    lines.skip_line();
                    count += 1;
                } else {
                    break;
                }
            }
        }

        if count > 1 {
            ReaderResult::Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                TokenValue::EmptyLines(count),
            ))
        } else {
            ReaderResult::None
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<MyToken, MyError> for WhitespaceReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        1usize
    }

    fn read(
        &self,
        _: &Readers<MyToken, MyError>,
        input: &mut Input,
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
pub struct EmptyReader;

impl Reader<MyToken, MyError> for EmptyReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        2usize
    }

    fn read(
        &self,
        _: &Readers<MyToken, MyError>,
        input: &mut Input,
        _: &State,
        next: &mut State,
    ) -> ReaderResult<MyToken, MyError> {
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

                if string.as_str() == "EMPTY" {
                    ReaderResult::Empty
                } else {
                    ReaderResult::None
                }
            } else {
                ReaderResult::None
            },
            None => ReaderResult::None,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<MyToken, MyError> for IdentifierReader {
    #[inline(always)]
    fn priority(&self) -> usize {
        3usize
    }

    fn read(
        &self,
        _: &Readers<MyToken, MyError>,
        input: &mut Input,
        current: &State,
        next: &mut State,
    ) -> ReaderResult<MyToken, MyError> {
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

#[test]
fn test_lexer_newlines() {
    let readers = ReadersBuilder::new()
        .add(WhitespaceReader)
        .add(EmptyReader)
        .add(IdentifierReader)
        .add(EmptyLineReader)
        .build();

    let lexer = readers.lexer("\n\n\nHello\n".chars());
    let tokens: Vec<MyToken> = lexer.map(|t| t.unwrap()).collect();

    assert_eq!(tokens.len(), 3);

    let ws_token = &tokens[0];
    assert_eq!(ws_token.value(), &TokenValue::EmptyLines(3));
    assert_eq!(ws_token.meta().col_start(), 1);
    assert_eq!(ws_token.meta().col_end(), 1);
    assert_eq!(ws_token.meta().col_count(), 1);
    assert_eq!(ws_token.meta().line_start(), 1);
    assert_eq!(ws_token.meta().line_end(), 4);
    assert_eq!(ws_token.meta().line_count(), 3);
    assert_eq!(ws_token.meta().len(), 3);
}

#[test]
fn test_lexer_whitespace() {
    let readers = ReadersBuilder::new()
        .add(WhitespaceReader)
        .add(EmptyReader)
        .add(IdentifierReader)
        .add(EmptyLineReader)
        .build();

    let lexer = readers.lexer("EMPTY   \n\t   EMPTY".chars());
    let tokens: Vec<MyToken> = lexer.map(|t| t.unwrap()).collect();

    assert_eq!(tokens.len(), 1);

    let ws_token = &tokens[0];
    assert_eq!(
        ws_token.value(),
        &TokenValue::Whitespace("   \n\t   ".into())
    );
    assert_eq!(ws_token.meta().col_start(), 5);
    assert_eq!(ws_token.meta().col_end(), 5);
    assert_eq!(ws_token.meta().col_count(), 1);
    assert_eq!(ws_token.meta().line_start(), 1);
    assert_eq!(ws_token.meta().line_end(), 2);
    assert_eq!(ws_token.meta().line_count(), 1);
    assert_eq!(ws_token.meta().len(), 8);
}

#[test]
fn test_lexer_identifier() {
    use std::fs::File;
    use std::io::Read;

    let readers = ReadersBuilder::new()
        .add(WhitespaceReader)
        .add(EmptyReader)
        .add(IdentifierReader)
        .add(EmptyLineReader)
        .build();

    let chars = File::open("tests/file.txt")
        .unwrap()
        .chars()
        .map(|r| match r {
            Ok(ch) => ch,
            Err(e) => panic!("{:?}", e),
        });

    let lexer = readers.lexer(chars);
    let tokens: Vec<MyToken> = lexer.map(|t| t.unwrap()).collect();

    assert_eq!(tokens.len(), 4);

    let ident_token = &tokens[0];
    assert_eq!(ident_token.value(), &TokenValue::Identifier("def".into()));
    assert_eq!(ident_token.meta().col_start(), 1);
    assert_eq!(ident_token.meta().col_end(), 3);
    assert_eq!(ident_token.meta().col_count(), 3);
    assert_eq!(ident_token.meta().line_start(), 1);
    assert_eq!(ident_token.meta().line_end(), 1);
    assert_eq!(ident_token.meta().line_count(), 0);
    assert_eq!(ident_token.meta().len(), 3);

    assert_eq!(tokens[1].value(), &TokenValue::Whitespace(" ".into()));
    assert_eq!(tokens[2].value(), &TokenValue::Identifier("name".into()));
    assert_eq!(tokens[3].value(), &TokenValue::Whitespace("\n".into()));
}
