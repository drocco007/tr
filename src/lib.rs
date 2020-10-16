use std::borrow::Cow;
use std::collections::HashMap;


/// Create a mapping from each char in set1 to the corresponding char
/// in set2.
///
/// # Examples
///
/// ```
/// let map = tr::map_charsets("abcde", "zyxwv");
///
/// assert_eq!(&'x', map.get(&'c').unwrap());
/// ```
pub fn map_charsets(set1: &str, set2: &str) -> HashMap<char, char> {
    set1.chars()
        .zip(rpad_last(set2, set1.len()).chars())
        .collect()
}


/// Extend s to length n by repeating the last char.
///
/// # Examples
///
/// ```
/// assert_eq!("Rust!!!", tr::rpad_last("Rust!", 7));
/// ```
///
/// Returns s unmodified if n is <= s.len():
///
/// ```
/// assert_eq!("why?", tr::rpad_last("why?", 0));
/// assert_eq!("too small", tr::rpad_last("too small", 4));
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
