use std::vec::Vec;
use std::iter::Iterator;

use token::Token;


#[derive(Debug)]
pub struct Lexer {
    index: usize,
    length: usize,
    chars: Vec<char>,
}

impl Lexer {
    #[inline(always)]
    pub fn new(string: String) -> Lexer {
        let chars: Vec<char> = string.chars().collect();

        Lexer {
            index: 0usize,
            length: chars.len(),
            chars: chars
        }
    }

    #[inline(always)]
    fn next_token(&mut self) -> Option<Token> {
        if self.index == self.length {
            None
        } else {
            self.parse_token()
        }
    }

    #[inline(always)]
    fn parse_token(&mut self) -> Option<Token> {
        let ch = self.chars.get(self.index).expect("the world is ending").clone();

        if ch.is_digit(10) {
            Some(self.parse_digit(ch))
        } else if ch.is_alphabetic() {
            Some(self.parse_alphabetic(ch))
        } else if is_special(ch) {
            Some(self.parse_special(ch))
        } else {
            self.index += 1;
            self.next_token()
        }
    }

    #[inline(always)]
    fn parse_digit(&mut self, ch: char) -> Token {
        let start = self.index;
        let mut index = start + 1;
        let mut parsed_period = false;
        let mut string = String::new();

        string.push(ch);

        while index < self.length {
            let ch = self.chars.get(index).expect("the world is ending").clone();

            if ch.is_numeric() {
                string.push(ch);
                index += 1;
            } else if parsed_period == false && ch == '.' {
                parsed_period = true;
                string.push(ch);
                index += 1;
            } else {
                break;
            }
        }

        self.index = index;

        Token::new(string, start, index)
    }

    #[inline(always)]
    fn parse_alphabetic(&mut self, ch: char) -> Token {
        let start = self.index;
        let mut index = start + 1;
        let mut string = String::new();

        string.push(ch);

        while index < self.length {
            let ch = self.chars.get(index).expect("the world is ending").clone();

            if ch.is_alphanumeric() {
                string.push(ch);
                index += 1;
            } else {
                break;
            }
        }

        self.index = index;

        Token::new(string, start, index)
    }

    #[inline(always)]
    fn parse_special(&mut self, ch: char) -> Token {
        let start = self.index;
        let index = start + 1;
        let mut string = String::new();

        string.push(ch);

        self.index = index;
        Token::new(string, start, index)
    }
}

#[inline(always)]
fn is_special(ch: char) -> bool {
    ch == '[' || ch == ']' ||
    ch == '(' || ch == ')' ||
    ch == '{' || ch == '}' ||
    ch == ',' || ch == ';' || ch == ':' ||
    ch == '?' || ch == '!' || ch == '~' ||
    ch == '+' || ch == '-' || ch == '*' || ch == '/' ||
    ch == '%' || ch == '^' || ch == '&' ||
    ch == '<' || ch == '>'
}

impl Iterator for Lexer {
    type Item = Token;

    #[inline(always)]
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
