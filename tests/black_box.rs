
use std::io::Cursor;

use tr::command::tr;


fn _tr<A>(args: A, stdin: &str) -> String
where
    A: IntoIterator,
    A::Item: AsRef<str>
{
    let stdin = Cursor::new(stdin);
    let mut stdout = Vec::new();

    tr(args, stdin, &mut stdout);

    String::from_utf8(stdout).expect("Not UTF-8")
}


#[test]
fn should_say_hello_loudly() {
    let output = _tr(vec!["tr", "a-z", "A-Z"], "hello world!");

    assert_eq!(output, "HELLO WORLD!");
}


#[test]
#[ignore]  // implement squeeze
fn should_squeeze_newlines() {
    let output = _tr(vec!["tr", "-s", "\n"], "\n\n\n");

    assert_eq!(output, "\n");
}


#[test]
fn should_translate_plain_suit_to_fancy() {
    let output = _tr(vec!["tr", "shdc", "♠♡♢♣"], "As Qh");

    assert_eq!(output, "A♠ Q♡");
}
