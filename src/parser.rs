use std::borrow::Cow;
use std::collections::HashMap;


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
/// assert_eq!("\n", tr::parser::unescape("\\n"));
/// assert_eq!("\n", tr::parser::unescape(r"\n"));
/// assert_eq!("x", tr::parser::unescape(r"\x"));
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


pub fn parse<'a>(s: &'a str) -> Cow<'a, str> {
    let (mut first, mut rest);

    if let Some(index) = s.find(r"\") {
        first = &s[..index];
        rest = &s[index..];
    } else {
        return s.into();
    }

    let mut output = String::with_capacity(s.len());

    loop {
        output.push_str(first);

        let c = &rest[..2];

        // consume the char from the input
        rest = &rest[2..];

        output.push_str(unescape(c));

        if let Some(index) = rest.find(r"\") {
            first = &rest[..index];
            rest = &rest[index..];
        } else {
            output.push_str(rest);
            break;
        }
    }

    output.into()
}
