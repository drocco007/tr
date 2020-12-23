use std::mem::replace;

use bstr::{ByteSlice};


#[derive(Debug)]
pub struct Lexer<'a> {
    s: &'a str,
    tokens: Vec<Token>,
    state: State,
}


#[derive(Debug,PartialEq)]
pub enum TokenType {
    Literal,
    CharRange,
    CharRepeat,
    CharClass,
    Equivalence,
}


#[derive(Debug,PartialEq)]
enum State {
    ScanLiteral,
    InterpretBackslash,
    InterpretBackslashOctal,
    RangePending,
}


#[derive(Debug,PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub token: String,
}


impl Token {
    fn new<S>(token_type: TokenType, token: S) -> Token where S: Into<String> {
        Token { token_type: token_type, token: token.into() }
    }
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if !self.tokens.is_empty() {
            return Some(self.tokens.remove(0));
        } else if self.s.is_empty() {
            return None;
        } else {
            self.scan();
            return self.next();
        }
    }
}


impl<'a> Lexer<'a> {
    fn emit(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn scan(&mut self) {
        use State::{*};

        let mut consumed = 0;
        let mut scanned = String::new();

        macro_rules! emit_prior {
             () => {
                if !scanned.is_empty() {
                    consumed = scanned.len();
                    self.emit(Token::new(TokenType::Literal, replace(& mut scanned, String::new())));
                }
            }
        };

        for c in self.s.as_bytes().graphemes() {
            match self.state {
                InterpretBackslash => {
                    match c.chars().next() {
                        Some('0'..='7') => {
                            scanned.push_str(c);
                            consumed += 1;
                            self.state = InterpretBackslashOctal;
                        },
                        _ => {
                            scanned.push_str(c);
                            consumed += 1;
                            self.emit(Token::new(TokenType::Literal, unescape(&scanned)));
                            break;
                        }
                    }
                },
                InterpretBackslashOctal => {
                    match c.chars().next() {
                        Some('0'..='7') if scanned.len() <= 3 => {
                            scanned.push_str(c);
                            consumed += 1;
                        },
                        _ => {
                            self.emit(Token::new(TokenType::Literal, octal_to_str(&scanned)));
                            break;
                        }
                    }
                },
                RangePending => {
                    // remove the dash
                    scanned.pop();

                    let first = scanned.pop().unwrap();

                    emit_prior!();

                    consumed += 3;
                    self.emit(Token::new(TokenType::CharRange, format!("{}-{}", first, c)));
                    break;
                },
                _ => {
                    match c {
                        "\\" => {
                            emit_prior!();
                            scanned.push_str(c);
                            consumed += 1;
                            self.state = InterpretBackslash;
                        },
                        "-" => {
                            if !scanned.is_empty() {
                                self.state = RangePending;
                            }

                            scanned.push_str(c);
                        },
                        "[" => {
                            let start = scanned.len();

                            let success = _is_equivalence(&self.s[start..])
                                .or_else(|| _is_repeat(&self.s[start..]))
                                .or_else(|| _is_class(&self.s[start..]));

                            if let Some((token, length)) = success {
                                emit_prior!();

                                self.emit(token);
                                consumed += length;

                                break;
                            } else {
                                scanned.push_str(c);
                            }
                        },
                        _ => { scanned.push_str(c); }
                    }
                }
            }
        }

        if self.tokens.is_empty() {
            consumed = scanned.len();
            self.emit(Token::new(TokenType::Literal, scanned));
        }

        self.s = &self.s[consumed..];
        self.state = ScanLiteral;
    }
}


// TODO: support Unicode repeats
fn _is_repeat(s: &str) -> Option<(Token, usize)> {
    use TokenType::{CharRepeat};

    if s.len() < 4 || &s[2..3] != "*" {
        return None;
    }

    for (i, c) in s[3..].chars().enumerate() {
        match c {
            ']' => { return Some((Token::new(CharRepeat, &s[..i+4]), i + 4)) },
            '0'..='9' => { continue; },
            _ => { break; }
        }
    }

    None
}


fn _is_equivalence(s: &str) -> Option<(Token, usize)> {
    use TokenType::{Equivalence};

    if s.len() >= 5 && &s[..2] == "[=" && &s[3..5] == "=]" {
        return Some((Token::new(Equivalence, &s[..5]), 5));
    }

    None
}


fn _is_class(s: &str) -> Option<(Token, usize)> {
    use TokenType::{CharClass};

    if s.len() >= 10 && &s[..10] == "[:xdigit:]" {
        return Some((Token::new(CharClass, "[:xdigit:]"), 10));
    } else if s.len() >= 9 {
        if &s[..2] == "[:" && &s[7..9] == ":]" {
            match &s[2..7] {
                "alnum" | "alpha" | "blank" | "cntrl" | "digit" | "graph" |
                "lower" | "print" | "punct" | "space" | "upper"
                => { return Some((Token::new(CharClass, &s[..9]), 9)); },
                _ => ()
            }
        }
    }

    None
}


/// Replace escape sequence with the corresponding char. s is assumed to
/// be a 2 character string
///
///    \\     backslash
///    \a     audible BEL
///    \b     backspace
///    \f     form feed
///    \n     new line
///    \r     return
///    \t     horizontal tab
///    \v     vertical tab
///
/// A backslash followed by any other char is replaced with that char; the
/// backslash is consumed and not reflected in the output.
///
/// # Examples
///
/// ```
/// assert_eq!("\n", tr::lex::unescape("\\n"));
/// assert_eq!("\n", tr::lex::unescape(r"\n"));
/// assert_eq!("x", tr::lex::unescape(r"\x"));
/// ```
pub fn unescape(s: &str) -> &str {
    match s {
        r"\a" => "\u{07}",
        r"\b" => "\u{08}",
        r"\f" => "\u{0c}",
        r"\n" => "\n",
        r"\r" => "\r",
        r"\t" => "\t",
        r"\v" => "\u{0b}",
        _ => &s[1..]
    }
}


fn octal_to_str(s: &str) -> char {
    let mut index = 0;

    if &s[..1] == r"\" {
        index = 1;
    }

    // FIXME!
    std::char::from_u32(u32::from_str_radix(&s[index..], 8).unwrap()).unwrap()
}


pub fn tokenize(s: &str) -> Lexer {
    Lexer { s: s, tokens: vec![], state: State::ScanLiteral }
}
