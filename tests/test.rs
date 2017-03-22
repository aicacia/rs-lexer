extern crate lexical_reader;


use lexical_reader::*;


#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TokenKind {
    WHITESPACE,
    IDENTIFIER,
}


#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct WhitespaceReader;

impl Reader<TokenKind> for WhitespaceReader {
    fn read(&self, input: &Input, state: &mut State) -> Option<Token<TokenKind>> {
        let ch = input.read(state);

        if ch.is_whitespace() {
            let mut string = String::new();

            string.push(ch);

            while !state.done() {
                let ch = input.char_at(state, 0);

                if ch.is_whitespace() {
                    input.read(state);
                    string.push(ch);
                } else {
                    break;
                }
            }

            Some(Token::new(
                input.new_state_meta(state),
                TokenKind::WHITESPACE,
                string
            ))
        } else {
            None
        }
    }
    #[inline(always)]
    fn priority(&self) -> usize {
        1usize
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct IdentifierReader;

impl Reader<TokenKind> for IdentifierReader {
    fn read(&self, input: &Input, state: &mut State) -> Option<Token<TokenKind>> {
        let ch = input.read(state);

        if ch.is_alphabetic() {
            let mut string = String::new();

            string.push(ch);

            while !state.done() {
                let ch = input.char_at(state, 0);

                if ch.is_alphanumeric() {
                    input.read(state);
                    string.push(ch);
                } else {
                    break;
                }
            }

            Some(Token::new(
                input.new_state_meta(state),
                TokenKind::IDENTIFIER,
                string
            ))
        } else {
            None
        }
    }
    #[inline(always)]
    fn priority(&self) -> usize {
        0usize
    }
}


#[test]
fn test_lexer_whitespace() {
    let mut lexer = LexicalReader::<TokenKind>::from("   \n\t   ");

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
    assert_eq!(ws_token.meta().line_start(), 1);
    assert_eq!(ws_token.meta().line_end(), 2);
    assert_eq!(ws_token.value().len(), 8);
}

#[test]
fn test_lexer_identifier() {
    let mut lexer = LexicalReader::<TokenKind>::from("def name");

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
    assert_eq!(ident_token.meta().line_start(), 1);
    assert_eq!(ident_token.meta().line_end(), 1);
    assert_eq!(ident_token.value(), &"def");
    assert_eq!(ident_token.value().len(), 3);
}
