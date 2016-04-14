use collections::string::String;
use collections::vec::Vec;
use core::iter::Iterator;

use token::Token;


#[derive(Debug)]
pub struct Lexer {
    syntex: String,
    index: usize,
    row: usize,
    column: usize,
    length: usize,
    chars: Vec<char>,
}

impl Lexer {
    pub fn new(string: String, syntex: String) -> Lexer {
        let chars: Vec<char> = string.chars().collect();

        Lexer {
            syntex: syntex,
            index: 0usize,
            row: 1usize,
            column: 1usize,
            length: chars.len(),
            chars: chars
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.index == self.length {
            None
        } else {
            let ch = self.read();

            if ch.is_whitespace() {
                self.next_token()
            } else if ch.is_digit(10) || ((ch == '-' || ch == '.') && self.char_at(0).is_digit(10)) {
                Some(self.read_digit(ch))
            } else if ch == '\'' || ch == '"' {
                Some(self.read_quoted(ch))
            } else if self.is_syntex(ch) {
                Some(self.read_syntex(ch))
            } else {
                Some(self.read_symbol(ch))
            }
        }
    }

    fn read(&mut self) -> char {
        let ch = self.char_at(0);

        if ch == '\n' || ch == '\r' {
            self.column += 1;
            self.row = 1;
        } else if self.index != 0 {
            self.row += 1;
        }

        if self.index < self.length {
            self.index += 1;
        }

        ch
    }

    fn read_size(&mut self, size: usize) -> String {
        let mut out = String::new();
        for _ in 0..size {
            out.push(self.read());
        }
        out
    }

    #[inline(always)]
    fn char_at(&self, index: usize) -> char {
        self.chars.get(self.index + index).expect("unexpected end of input").clone()
    }

    #[inline(always)]
    fn new_token(&self, string: String, token_type: String) -> Token {
        Token::new(string, token_type, self.row, self.column)
    }

    #[inline(always)]
    fn is_syntex(&self, ch: char) -> bool {
        self.syntex.contains(ch)
    }

    fn read_digit(&mut self, ch: char) -> Token {
        let mut index = self.index;
        let mut parsed_period = false;
        let mut parsed_hex = false;
        let mut string = String::new();

        if ch == '.' {
            parsed_period = true;
        }

        string.push(ch);

        while index < self.length {
            let ch = self.char_at(0);

            index += 1;

            if parsed_hex {
                if ch.is_digit(16) {
                    self.read();
                    string.push(ch);
                } else {
                    break;
                }
            } else {
                if ch.is_digit(10) {
                    self.read();
                    string.push(ch);
                } else if parsed_hex == false && ch == 'x' {
                    self.read();
                    parsed_hex = true;
                    string.push(ch);
                } else if parsed_period == false && ch == '.' {
                    self.read();
                    parsed_period = true;
                    string.push(ch);
                } else {
                    break;
                }
            }
        }

        self.new_token(string, String::from("number"))
    }

    fn read_quoted(&mut self, ch: char) -> Token {
        let quote = ch;
        let mut index = self.index;
        let mut escape = false;
        let mut string = String::new();

        while index < self.length {
            let ch = self.char_at(0);
            let mut count = 1;

            if escape {
                if ch == 'u' {
                    self.read();
                    let hex = self.read_size(4);
                    count = 4;
                    string.push(hex.parse::<u8>().unwrap() as char);
                } else {
                    self.read();
                    string.push(escape_char(ch));
                }
                escape = false;
            } else if ch == '\\' {
                self.read();
                escape = true;
            } else if ch == quote {
                self.read();
                break;
            } else {
                self.read();
                string.push(ch);
            }

            index += count;
        }

        self.new_token(string, string_from_char(quote))
    }

    fn read_symbol(&mut self, ch: char) -> Token {
        let mut index = self.index;
        let mut string = String::new();

        string.push(ch);

        while index < self.length {
            let ch = self.char_at(0);

            if ch.is_alphanumeric() || ch == '_' {
                string.push(ch);
                self.read();
                index += 1;
            } else {
                break;
            }
        }

        self.new_token(string, String::from("symbol"))
    }

    fn read_syntex(&mut self, ch: char) -> Token {
        self.new_token(string_from_char(ch), String::from("syntex"))
    }
}

#[inline(always)]
fn string_from_char(ch: char) -> String {
    let mut string = String::new();
    string.push(ch);
    string
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

impl Iterator for Lexer {
    type Item = Token;

    #[inline(always)]
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
