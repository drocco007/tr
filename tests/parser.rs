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
