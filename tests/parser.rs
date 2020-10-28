use rstest::rstest;

use tr::parser::parse;


#[test]
fn backslash_should_produce_backslash() {
    assert_eq!(r"\", parse(r"\\"));
}


#[test]
fn backslash_8_should_produce_8() {
    assert_eq!("8", parse(r"\8"));
}


#[test]
fn backslash_9_should_produce_9() {
    assert_eq!("9", parse(r"\9"));
}


#[test]
fn bel_escape_should_produce_bel() {
    assert_eq!("\u{07}", parse(r"\a"));
}


#[test]
fn backspace_escape_should_produce_backspace() {
    assert_eq!("\u{08}", parse(r"\b"));
}


#[test]
fn formfeed_escape_should_produce_formfeed() {
    assert_eq!("\u{0c}", parse(r"\f"));
}


#[test]
fn newline_escape_should_produce_newline() {
    assert_eq!("\n", parse(r"\n"));
}


#[test]
fn cr_escape_should_produce_cr() {
    assert_eq!("\r", parse(r"\r"));
}


#[test]
fn tab_escape_should_produce_tab() {
    assert_eq!("\t", parse(r"\t"));
}


#[test]
fn vertical_tab_escape_should_produce_vertical_tab() {
    assert_eq!("\u{0b}", parse(r"\v"));
}


#[rstest(
    case => [("a-g", "abcdefg"), (" -/", " !\"#$%&'()*+,-./"),
             ("0-9", "0123456789"), ("2-5", "2345"), ("B-D6-8", "BCD678"),
             ("9-@", "9:;<=>?@")]
)]
fn character_range_should_produce_characters(case: (&str, &str)) {
    let (range, expected) = case;

    assert_eq!(expected, parse(range));
}


#[test]
fn character_range_used_as_verbose_spelling_for_character() {
    assert_eq!("5", parse("5-5"));
}


#[test]
fn alnum_class_should_expand() {
    assert_eq!("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
               parse("[:alnum:]"));
}


#[test]
fn alpha_class_should_expand() {
    assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz",
               parse("[:alpha:]"));
}


#[test]
fn blank_class_should_expand() {
    assert_eq!("\t ", parse("[:blank:]"));
}


#[test]
fn cntrl_class_should_expand() {
    assert_eq!("\u{0}\u{1}\u{2}\u{3}\u{4}\u{5}\u{6}\u{7}\u{8}\t\n\u{b}\u{c}\r\u{e}\u{f}\u{10}\u{11}\u{12}\u{13}\u{14}\u{15}\u{16}\u{17}\u{18}\u{19}\u{1a}\u{1b}\u{1c}\u{1d}\u{1e}\u{1f}", parse("[:cntrl:]"));
}


#[test]
fn digit_class_should_expand() {
    assert_eq!("0123456789", parse("[:digit:]"));
}


#[test]
fn graph_class_should_expand() {
    assert_eq!("!\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
               parse("[:graph:]"));
}


#[test]
fn lower_class_should_expand() {
    assert_eq!("abcdefghijklmnopqrstuvwxyz", parse("[:lower:]"));
}


#[test]
fn print_class_should_expand() {
    assert_eq!(" !\"#$%&\'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~",
               parse("[:print:]"));
}


#[test]
fn punct_class_should_expand() {
    // $ python3 -c 'for i in range(128): print(chr(i), end="")' | tr -cd '[:punct:]'
    assert_eq!("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~", parse("[:punct:]"));
}


#[test]
fn space_class_should_expand() {
    assert_eq!("\t\n\u{b}\u{c}\r ", parse("[:space:]"));
}


#[test]
fn upper_class_should_expand() {
    assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ", parse("[:upper:]"));
}


#[test]
fn xdigit_class_should_expand() {
    assert_eq!("0123456789ABCDEFabcdef", parse("[:xdigit:]"));
}
