#[derive(Debug)]
pub struct Lexer<'a> {
    s: &'a str,
    next_token: Option<&'a str>,
}


impl<'a> Iterator for Lexer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<&'a str> {
        if self.next_token != None {
            let result = self.next_token;
            self.next_token = None;
            return result;
        } else if self.s.is_empty() {
            return None;
        }

        let mut result = self.s;
        let mut consumed = 0;

        for (i, c) in self.s.chars().enumerate() {
            match c {
                '\\' => {
                    if i == 0 {
                        result = _tokenize_backslash(self.s);
                        consumed = result.len();
                    } else {
                        result = &self.s[..i];
                        let next_token = _tokenize_backslash(&self.s[i..]);
                        consumed = i+next_token.len();
                        self.next_token = Some(next_token);
                    }

                    break;
                },
                '-' => {
                    if i == 0 || i == self.s.len() - 1 {
                        consumed += 1;
                        continue;
                    } else if i == 1 {
                        result = &self.s[..i+2];
                        consumed += 2;
                    } else {
                        result = &self.s[..i-1];
                        self.next_token = Some(&self.s[i-1..i+2]);
                        consumed += 2;
                    }

                    break;
                },
                '[' => {
                    if let Some(j) = _is_equivalence(&self.s[i..]).or_else(|| _is_repeat(&self.s[i..])).or_else(|| _is_class(&self.s[i..])) {
                        if i == 0 {
                            result = &self.s[..j];
                        } else {
                            result = &self.s[..i];
                            self.next_token = Some(&self.s[i..i+j]);
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


fn _is_repeat(s: &str) -> Option<usize> {
    if s.len() < 4 || &s[2..3] != "*" {
        return None;
    }

    for (i, c) in s[3..].chars().enumerate() {
        match c {
            ']' => { return Some(i + 4); },
            '0'..='9' => { continue; },
            _ => { break; }
        }
    }

    None
}


fn _is_equivalence(s: &str) -> Option<usize> {
    if s.len() >= 5 && &s[..2] == "[=" && &s[3..5] == "=]" {
        return Some(5);
    }

    None
}


fn _is_class(s: &str) -> Option<usize> {
    if s.len() >= 10 && &s[..10] == "[:xdigit:]" {
        return Some(10);
    } else if s.len() >= 9 {
        if &s[..2] == "[:" && &s[7..9] == ":]" {
            match &s[2..7] {
                "alnum" | "alpha" | "blank" | "cntrl" | "digit" | "graph" |
                "lower" | "print" | "punct" | "space" | "upper"
                => { return Some(9); },
                _ => ()
            }
        }
    }

    None
}


fn _tokenize_backslash(s: &str) -> &str {
    let mut chars = s.chars();

    chars.next();

    match chars.next() {
        Some('0'..='7') => (),
        Some(_) => { return &s[..2]; },
        None => { return s; }
    }

    let mut i = 2;

    for c in chars {
        match c {
            '0'..='7' if i <= 3 => { i += 1 },
            _ => { break; }
        }
    }

    &s[..i]
}


pub fn tokenize(s: &str) -> Lexer {
    Lexer { s: s, next_token: None }
}
