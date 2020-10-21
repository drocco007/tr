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


/// Replace escape sequences in s with the corresponding char.
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
/// assert_eq!("\n", tr::parser::parse("\\n"));
/// assert_eq!("\n", tr::parser::parse(r"\n"));
/// assert_eq!("x", tr::parser::parse(r"\x"));
/// ```
pub fn parse<'a>(s: &'a str) -> Cow<'a, str> {
    let (mut first, mut rest);

    if let Some(index) = s.find(r"\") {
        first = &s[..index];

        // index+1 -> skip the backslash
        rest = &s[index+1..];
    } else {
        return s.into();
    }

    let mut output = String::with_capacity(s.len());

    loop {
        output.push_str(first);

        let c = &rest[..1];

        // consume the char from the input
        rest = &rest[1..];

        match c {
            "a" => output.push_str("\u{07}"),
            "b" => output.push_str("\u{08}"),
            "f" => output.push_str("\u{0c}"),
            "n" => output.push_str("\n"),
            "r" => output.push_str("\r"),
            "t" => output.push_str("\t"),
            "v" => output.push_str("\u{0b}"),
            _ => output.push_str(c)
        }

        if let Some(index) = rest.find(r"\") {
            first = &rest[..index];

            // skip the backslash
            rest = &rest[index+1..];
        } else {
            output.push_str(rest);
            break;
        }
    }

    output.into()
}
