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
            let ch = self.char_at(0);

            if ch.is_digit(10) || ch == '.' && self.has_char(1) && self.char_at(1).is_digit(10) {
                Some(self.parse_digit(ch))
            } else if ch == '"' || ch == '\'' {
                Some(self.parse_string(ch))
            } else if ch.is_alphabetic() || ch == '_' {
                Some(self.parse_alphabetic(ch))
            } else if is_syntex(ch) {
                Some(self.parse_syntex(ch))
            } else if ch.is_whitespace() {
                self.index += 1;
                self.next_token()
            } else {
                self.parse_operator(ch)
            }
        }
    }

    #[inline(always)]
    fn parse_operator(&mut self, ch: char) ->  Option<Token> {
        let start = self.index;

        if self.has_char(1) {
            let ch2 = self.char_at(1);

            if self.has_char(2) {
                let ch3 = self.char_at(2);
                let op1 = is_operator1(ch);
                let op2 = is_operator2(ch, ch2);
                let op3 = is_operator3(ch, ch2, ch3);

                if op1 || op2 || op3 {
                    self.index += 2;
                    Some(Token::new(
                        if op3 {string_from_char3(ch, ch2, ch3)}
                        else if op2 {string_from_char2(ch, ch2)}
                        else {string_from_char(ch)},
                        String::from("operator"), start, start + 2)
                    )
                } else {
                    None
                }
            } else {
                let op1 = is_operator1(ch);
                let op2 = is_operator2(ch, ch2);

                if op1 || op2 {
                    self.index += 2;
                    Some(Token::new(
                        if op2 {string_from_char2(ch, ch2)}
                        else {string_from_char(ch)},
                        String::from("operator"), start, start + 2)
                    )
                } else {
                    None
                }
            }
        } else {
            if is_operator1(ch) {
                self.index += 1;
                Some(Token::new(string_from_char(ch), String::from("operator"), start, start + 1))
            } else {
                None
            }
        }
    }

    #[inline(always)]
    fn char_at(&mut self, index: usize) -> char {
        self.chars.get(self.index + index).expect("the world is ending").clone()
    }

    #[inline(always)]
    fn has_char(&mut self, index: usize) -> bool {
        self.index + index < self.length
    }

    #[inline(always)]
    fn parse_digit(&mut self, ch: char) -> Token {
        let start = self.index;
        let mut index = 1;
        let mut new_index = start + 1;
        let mut parsed_period = false;
        let mut string = String::new();

        string.push(ch);

        while new_index < self.length {
            let ch = self.char_at(index);

            if ch.is_numeric() {
                string.push(ch);
            } else if parsed_period == false && ch == '.' {
                parsed_period = true;
                string.push(ch);
            } else {
                break;
            }

            index += 1;
            new_index = start + index;
        }

        self.index = new_index;

        Token::new(string, String::from("number"), start, new_index)
    }

    #[inline(always)]
    fn parse_string(&mut self, ch: char) -> Token {
        let start = self.index;
        let quote = ch;
        let mut index = 1;
        let mut new_index = start + index;
        let mut escape = false;
        let mut string = String::new();

        while new_index < self.length {
            let ch = self.char_at(index);

            if escape {
                if ch == 'u' {
                    let hex = slice_string(&self.chars, index + 1, index + 5);
                    index += 4;
                    string.push(hex.parse::<u8>().unwrap() as char);
                } else {
                    index += 1;
                    string.push(escape_char(ch));
                }
                escape = false;
            } else if ch == '\\' {
                index += 1;
                escape = true;
            } else if ch == quote {
                index += 1;
                new_index = start + index;
                break;
            } else {
                index += 1;
                string.push(ch);
            }

            new_index = start + index;
        }

        self.index = new_index;

        Token::new(string, String::from("string"), start, new_index)
    }

    #[inline(always)]
    fn parse_alphabetic(&mut self, ch: char) -> Token {
        let start = self.index;
        let mut index = 1;
        let mut new_index = start + index;
        let mut string = String::new();

        string.push(ch);

        while index < self.length {
            let ch = self.char_at(index);

            if ch.is_alphanumeric() || ch == '_' {
                string.push(ch);
                index += 1;
                new_index = start + index;
            } else {
                break;
            }
        }

        self.index = new_index;

        Token::new(string, String::from("name"), start, new_index)
    }

    #[inline(always)]
    fn parse_syntex(&mut self, ch: char) -> Token {
        let start = self.index;
        let index = start + 1;
        self.index = index;
        Token::new(string_from_char(ch), String::from("syntex"), start, index)
    }
}

fn string_from_char(ch: char) -> String {
    let mut string = String::new();
    string.push(ch);
    string
}

fn string_from_char2(ch: char, ch2: char) -> String {
    let mut string = String::new();
    string.push(ch);
    string.push(ch2);
    string
}

fn string_from_char3(ch: char, ch2: char, ch3: char) -> String {
    let mut string = String::new();
    string.push(ch);
    string.push(ch2);
    string.push(ch3);
    string
}

fn is_operator1(ch: char) -> bool {
    ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '%' || ch == '!' || ch == '=' || ch == '|'
}

fn is_operator2(ch: char, ch2: char) -> bool {
    ch == '=' && ch2 == '=' ||
    ch == '!' && ch2 == '=' ||
    ch == '<' && ch2 == '=' ||
    ch == '>' && ch2 == '=' ||
    ch == '&' && ch2 == '&' ||
    ch == '|' && ch2 == '|'
}

fn is_operator3(ch: char, ch2: char, ch3: char) -> bool {
    ch == '=' && ch2 == '=' && ch3 == '=' ||
    ch == '!' && ch2 == '=' && ch3 == '='
}

fn slice_string(chars: &Vec<char>, start: usize, end: usize) -> String {
    let mut out = String::new();
    for i in start..end {
        out.push(chars.get(i).unwrap().clone());
    }
    out
}

#[inline(always)]
fn escape_char(ch: char) -> char {
    match ch {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '\'' => '\'',
        '"' => '"',
        _ => ch
    }
}

#[inline(always)]
fn is_syntex(ch: char) -> bool {
    ch == '[' || ch == ']' ||
    ch == '(' || ch == ')' ||
    ch == '{' || ch == '}' ||
    ch == '.' || ch == ',' || ch == ';' || ch == ':' || ch == '?'
}

impl Iterator for Lexer {
    type Item = Token;

    #[inline(always)]
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
