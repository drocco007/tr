#[derive(Debug)]
pub struct Lexer<'a> {
    s: &'a str,
    next_token: Option<Token<'a>>,
}


#[derive(Debug,PartialEq)]
pub enum TokenType {
    Literal,
    BackslashEscape,
    OctalEscape,
    CharRange,
    CharRepeat,
    CharClass,
    Equivalence,
}


#[derive(Debug,PartialEq)]
pub struct Token<'a> {
    pub token_type: TokenType,
    pub token: &'a str,
}


impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        if self.next_token != None {
            let result = std::mem::replace(&mut self.next_token, None);
            return result;
        } else if self.s.is_empty() {
            return None;
        }

        let mut result = Token {token_type: TokenType::Literal, token: self.s };
        let mut consumed = 0;

        for (i, c) in self.s.chars().enumerate() {
            match c {
                '\\' => {
                    if i == 0 {
                        let (token, token_type) = _tokenize_backslash(self.s);
                        result.token = token;
                        result.token_type = token_type;
                        consumed = result.token.len();
                    } else {
                        result.token = &self.s[..i];
                        let (token, token_type) = _tokenize_backslash(&self.s[i..]);
                        consumed = i+token.len();
                        self.next_token = Some(Token { token_type: token_type, token: token } );
                    }

                    break;
                },
                '-' => {
                    if i == 0 || i == self.s.len() - 1 {
                        consumed += 1;
                        continue;
                    } else if i == 1 {
                        result.token = &self.s[..i+2];
                        result.token_type = TokenType::CharRange;
                        consumed += 2;
                    } else {
                        result.token = &self.s[..i-1];
                        self.next_token = Some(Token { token_type: TokenType::CharRange, token: &self.s[i-1..i+2] } );
                        consumed += 2;
                    }

                    break;
                },
                '[' => {
                    if let Some((j, token_type)) = _is_equivalence(&self.s[i..]).or_else(|| _is_repeat(&self.s[i..])).or_else(|| _is_class(&self.s[i..])) {
                        if i == 0 {
                            result.token = &self.s[..j];
                            result.token_type = token_type;
                        } else {
                            result.token = &self.s[..i];
                            self.next_token = Some(Token { token_type: token_type, token: &self.s[i..i+j] } );
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


fn _tokenize_backslash(s: &str) -> (&str, TokenType) {
    let mut chars = s.chars();

    chars.next();

    match chars.next() {
        Some('0'..='7') => (),
        Some(_) => { return (&s[..2], TokenType::BackslashEscape); },
        None => { return (s, TokenType::Literal); }
    }

    let mut i = 2;

    for c in chars {
        match c {
            '0'..='7' if i <= 3 => { i += 1 },
            _ => { break; }
        }
    }

    (&s[..i], TokenType::OctalEscape)
}


pub fn tokenize(s: &str) -> Lexer {
    Lexer { s: s, next_token: None }
}
