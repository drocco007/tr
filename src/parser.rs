use std::borrow::Cow;
use std::collections::HashMap;

use bstr::{ByteSlice};

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
/// assert_eq!(&"x", map.get("c").unwrap());
/// ```
pub fn map_charsets<'a>(set1: &'a str, set2: &'a str) -> HashMap<String, String> {
    parse(set1).as_bytes().graphemes().zip(parse(set2).as_bytes().graphemes())
        .map(|(c1, c2)| (c1.to_string(), c2.to_string()))
        .collect()
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


fn expand_range(s: &str) -> String {
    let s = s.chars().collect::<Vec<char>>();

    std::ops::RangeInclusive::new(s[0], s[2]).collect::<String>()
}


fn expand_class(s: &str) -> String {
    match s {
        "[:alnum:]" => "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        "[:alpha:]" => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
        "[:blank:]" => "\t ",

        // python3 -c 'for i in range(32): print(chr(i), end="")' | tr '[:cntrl:]' '.'
        "[:cntrl:]" => "\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\n\u{b}\u{c}\r\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f}\u{7f}",
        "[:digit:]" => "0123456789",
        "[:graph:]" => "!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
        "[:lower:]" => "abcdefghijklmnopqrstuvwxyz",

        // apparently tab is not in this list: echo -ne '\t' | tr '[:print:]' '.' | xxd
        // 00000000: 09
        "[:print:]" => " !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
        "[:punct:]" => "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
        "[:space:]" => "\t\n\u{b}\u{c}\r ",
        "[:upper:]" => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
        "[:xdigit:]" => "0123456789ABCDEFabcdef",
        _ => panic!("tried to expand non class {:?}", s)
    }.into()
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
            CharRange => output.push_str(&expand_range(&token.token)),
            CharClass => output.push_str(&expand_class(&token.token)),
            _ => ()
        }
    }

    output.into()
}
