use collections::vec::Vec;
use collections::string::String;

use super::kind::Kind;
use super::token::Token;


#[derive(Debug)]
pub struct Lexer {
    index: usize,
    row: usize,
    column: usize,
    length: usize,
    chars: Vec<char>,
}

impl Lexer {
    #[inline]
    pub fn new(string: &str) -> Lexer {
        let chars: Vec<char> = string.chars().collect();

        Lexer {
            index: 0usize,
            row: 1usize,
            column: 1usize,
            length: chars.len(),
            chars: chars
        }
    }
    #[inline]
    pub fn next_token(&mut self) -> Option<Token> {
        if self.index == self.length {
            None
        } else {
            let start_index = self.index;
            let ch = self.read();
            let row = self.row;
            let column = self.column;

            if ch.is_whitespace() || ch == ',' {
                self.next_token()
            } else if ch == '"' || ch == '\'' {
                Some(self.read_quoted(ch, start_index, row, column))
            } else if ch.is_digit(10) || (
                (ch == '-' || ch == '.') &&
                self.has_char_at(0) &&
                self.char_at(0).is_digit(10)
            ) {
                Some(self.read_number(ch, start_index, row, column))
            } else if ch.is_alphabetic() || ch == '_' {
                Some(self.read_symbol(ch, start_index, row, column))
            } else {
                Some(self.read_syntax(ch, start_index, row, column))
            }
        }
    }
    #[inline(always)]
    pub fn has_next_token(&self) -> bool {
        self.index < self.length - 1
    }
    #[inline]
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
    #[inline(always)]
    fn char_at(&self, index: usize) -> char {
        *self.chars.get(self.index + index).expect("unexpected end of input")
    }
    #[inline(always)]
    fn has_char_at(&self, index: usize) -> bool {
        (self.index + index) < self.length
    }
    #[inline]
    fn read_size(&mut self, size: usize) -> String {
        let mut out = String::new();

        for _ in 0..size {
            out.push(self.read());
        }

        out
    }
    #[inline]
    fn read_quoted(&mut self, ch: char, start_index: usize, row: usize, column: usize) -> Token {
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
                    string.push(Self::escape_char(ch));
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

        let kind = if quote == '\'' {
            Kind::Chr
        } else {
            Kind::Str
        };

        Token::new(string, kind, start_index, row, column)
    }
    #[inline]
    fn read_number(&mut self, ch: char, start_index: usize, row: usize, column: usize) -> Token {
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

        Token::new(string, Kind::Num, start_index, row, column)
    }
    #[inline]
    fn read_symbol(&mut self, ch: char, start_index: usize, row: usize, column: usize) -> Token {
        let mut index = self.index;
        let mut string = String::new();

        string.push(ch);

        if ch.is_alphanumeric() || ch == '_' {
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
        }

        Token::new(string, Kind::Sym, start_index, row, column)
    }
    #[inline]
    fn read_syntax(&mut self, ch: char, start_index: usize, row: usize, column: usize) -> Token {
        let mut string = String::new();
        string.push(ch);
        Token::new(string, Kind::Syn, start_index, row, column)
    }
    #[inline]
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
}

impl Iterator for Lexer {
    type Item = Token;

    #[inline(always)]
    fn next(&mut self) -> Option<Token> {
        self.next_token()
    }
}
