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
        .zip(set2.chars())
        .collect()
}
