use std::borrow::Cow;
use std::collections::HashMap;

use crate::lex::tokenize;
use crate::lex::TokenType::{*};


/// Create a mapping from each char in set1 to the corresponding char
/// in set2.
///
/// # Examples
///
/// ```
/// let map = tr::parser::map_charsets("abcde", "zyxwv");
///
/// assert_eq!(&'x', map.get(&'c').unwrap());
/// ```
pub fn map_charsets(set1: &str, set2: &str) -> HashMap<char, char> {
    let (set1, set2) = (parse(set1), parse(set2));

    let set2 = rpad_last(&set2, set1.len());

    set1.chars().zip(set2.chars()).collect()
}


/// Extend s to length n by repeating the last char.
///
/// # Examples
///
/// ```
/// assert_eq!("Rust!!!", tr::parser::rpad_last("Rust!", 7));
/// ```
///
/// Returns s unmodified if n is <= s.len():
///
/// ```
/// assert_eq!("why?", tr::parser::rpad_last("why?", 0));
/// assert_eq!("too small", tr::parser::rpad_last("too small", 4));
/// ```
pub fn rpad_last<'a>(s: &'a str, n: usize) -> Cow<'a, str> {
    if s.len() < n {
        let mut buf = String::with_capacity(n);
        let n = n - s.len();
        let c = s.chars().rev().nth(0).expect("empty source string");

        buf.push_str(s);
        buf.push_str(&c.to_string().repeat(n));

        buf.into()
    } else {
        s.into()
    }
}


pub fn parse<'a>(s: &'a str) -> Cow<'a, str> {
    if s.is_empty() {
        return s.into();
    }

    let mut tokens = tokenize(s);
    let token = tokens.next().unwrap();

    if token.token_type == Literal && token.token == s {
        return s.into();
    }

    let mut output = String::with_capacity(s.len());

    for token in std::iter::once(token).chain(tokens) {
        match token.token_type {
            Literal => output.push_str(&token.token),
            _ => ()
        }
    }

    output.into()
}
