use std::borrow::Cow;


#[derive(Debug)]
pub struct Lexer<'a> {
    s: &'a str,
    next_token: Option<Token>,
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
pub struct Token {
    pub token_type: TokenType,
    pub token: String,
}


impl Token {
    fn new<S>(token_type: TokenType, token: S) -> Token where S: Into<String> {
        Token { token_type: token_type, token: token.into() }
    }
}


impl<'a> Lexer<'a> {
    fn _find_next(&mut self) -> Option<Token> {
        let mut result = Token::new(TokenType::Literal, self.s);
        let mut consumed = 0;

        for (i, c) in self.s.chars().enumerate() {
            match c {
                '\\' => {
                    if i == 0 {
                        let (token, token_type, length) = _tokenize_backslash(self.s);
                        result = Token::new(token_type, token);
                        consumed = length;
                    } else {
                        result = Token::new(TokenType::Literal, &self.s[..i]);
                    }

                    break;
                },
                '-' => {
                    if i == 0 || i == self.s.len() - 1 {
                        consumed += 1;
                        continue;
                    } else if i == 1 {
                        result = Token::new(TokenType::CharRange, &self.s[..i+2]);
                        consumed += 2;
                    } else {
                        result = Token::new(TokenType::Literal, &self.s[..i-1]);
                        self.next_token = Some(Token::new(TokenType::CharRange, &self.s[i-1..i+2]));
                        consumed += 2;
                    }

                    break;
                },
                '[' => {
                    if let Some((j, token_type)) = _is_equivalence(&self.s[i..]).or_else(|| _is_repeat(&self.s[i..])).or_else(|| _is_class(&self.s[i..])) {
                        if i == 0 {
                            result = Token::new(token_type, &self.s[..j]);
                        } else {
                            result = Token::new(TokenType::Literal, &self.s[..i]);
                            self.next_token = Some(Token::new(token_type, &self.s[i..i+j]));
                        }

                        consumed += j;
                        break;
                    } else {
                        consumed += 1;
                    }
                },
                _ => { consumed += 1; }
            }
        }

        self.s = &self.s[consumed..];

        Some(result)
    }
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        if self.next_token != None {
            return std::mem::replace(&mut self.next_token, None);
        } else if self.s.is_empty() {
            return None;
        } else {
            return self._find_next();
        }
    }
}


fn _is_repeat(s: &str) -> Option<(usize, TokenType)> {
    if s.len() < 4 || &s[2..3] != "*" {
        return None;
    }

    for (i, c) in s[3..].chars().enumerate() {
        match c {
            ']' => { return Some((i + 4, TokenType::CharRepeat)); },
            '0'..='9' => { continue; },
            _ => { break; }
        }
    }

    None
}


fn _is_equivalence(s: &str) -> Option<(usize, TokenType)> {
    if s.len() >= 5 && &s[..2] == "[=" && &s[3..5] == "=]" {
        return Some((5, TokenType::Equivalence));
    }

    None
}


fn _is_class(s: &str) -> Option<(usize, TokenType)> {
    if s.len() >= 10 && &s[..10] == "[:xdigit:]" {
        return Some((10, TokenType::CharClass));
    } else if s.len() >= 9 {
        if &s[..2] == "[:" && &s[7..9] == ":]" {
            match &s[2..7] {
                "alnum" | "alpha" | "blank" | "cntrl" | "digit" | "graph" |
                "lower" | "print" | "punct" | "space" | "upper"
                => { return Some((9, TokenType::CharClass)); },
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


fn _tokenize_backslash<'a>(s: &'a str) -> (Cow<'a, str>, TokenType, usize) {
    let mut chars = s.chars();

    chars.next();

    match chars.next() {
        Some('0'..='7') => (),
        Some(_) => { return (unescape(&s[..2]).into(), TokenType::Literal, 2); },
        None => { return (s.into(), TokenType::Literal, 1); }
    }

    let mut i = 2;

    for c in chars {
        match c {
            '0'..='7' if i <= 3 => { i += 1 },
            _ => { break; }
        }
    }

    (octal_to_str(&s[..i]).to_string().into(), TokenType::Literal, i)
}


pub fn tokenize(s: &str) -> Lexer {
    Lexer { s: s, next_token: None }
}
