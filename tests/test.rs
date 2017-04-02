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

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<TokenKind> for IdentifierReader {

    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }

    fn read(&self, input: &Input, current: &State, next: &mut State) -> Option<Token<TokenKind>> {
        let ch = input.read(next);

        if ch.is_alphabetic() {
            let mut string = String::new();

            string.push(ch);

            while !input.done(next) {
                let ch = input.peek(next, 0);

                if ch.is_alphanumeric() {
                    input.read(next);
                    string.push(ch);
                } else {
                    break;
                }
            }

            Some(Token::new(
                TokenMeta::new_state_meta(current, next),
                TokenKind::IDENTIFIER,
                string
            ))
        } else {
            None
        }
    }
}


#[test]
fn test_lexer_whitespace() {
    let mut lexer = Lexer::<TokenKind, _>::from("   \n\t   ");

    lexer.readers
        .add(WhitespaceReader)
        .add(IdentifierReader)
        .sort();

    let tokens: Vec<Token<TokenKind>> = lexer.collect();

    assert_eq!(tokens.len(), 1);

    let ws_token = &tokens[0];
    assert_eq!(ws_token.kind(), &TokenKind::WHITESPACE);
    assert_eq!(ws_token.meta().col_start(), 1);
    assert_eq!(ws_token.meta().col_end(), 5);
    assert_eq!(ws_token.meta().col_count(), 5);
    assert_eq!(ws_token.meta().line_start(), 1);
    assert_eq!(ws_token.meta().line_end(), 2);
    assert_eq!(ws_token.meta().line_count(), 2);
    assert_eq!(ws_token.meta().len(), 8);
    assert_eq!(ws_token.value().len(), 8);
}

#[test]
fn test_lexer_identifier() {
    let mut lexer = Lexer::<TokenKind, _>::from("def name");

    lexer.readers
        .add(WhitespaceReader)
        .add(IdentifierReader)
        .sort();

    let tokens: Vec<Token<TokenKind>> = lexer.collect();

    assert_eq!(tokens.len(), 3);

    let ident_token = &tokens[0];
    assert_eq!(ident_token.kind(), &TokenKind::IDENTIFIER);
    assert_eq!(ident_token.meta().col_start(), 1);
    assert_eq!(ident_token.meta().col_end(), 3);
    assert_eq!(ident_token.meta().col_count(), 3);
    assert_eq!(ident_token.meta().line_start(), 1);
    assert_eq!(ident_token.meta().line_end(), 1);
    assert_eq!(ident_token.meta().line_count(), 1);
    assert_eq!(ident_token.value(), "def");
    assert_eq!(ident_token.meta().len(), 3);
    assert_eq!(ident_token.value().len(), 3);
}
