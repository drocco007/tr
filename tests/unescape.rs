use tr::unescape;


#[test]
fn backslash_should_produce_backslash() {
    assert_eq!(r"\", unescape(r"\\"));
}


#[test]
fn backslash_8_should_produce_8() {
    assert_eq!("8", unescape(r"\8"));
}


#[test]
fn backslash_9_should_produce_9() {
    assert_eq!("9", unescape(r"\9"));
}


#[test]
fn bel_escape_should_produce_bel() {
    assert_eq!("\u{07}", unescape(r"\a"));
}


#[test]
fn backspace_escape_should_produce_backspace() {
    assert_eq!("\u{08}", unescape(r"\b"));
}


#[test]
fn formfeed_escape_should_produce_formfeed() {
    assert_eq!("\u{0c}", unescape(r"\f"));
}


#[test]
fn newline_escape_should_produce_newline() {
    assert_eq!("\n", unescape(r"\n"));
}


#[test]
fn cr_escape_should_produce_cr() {
    assert_eq!("\r", unescape(r"\r"));
}


#[test]
fn tab_escape_should_produce_tab() {
    assert_eq!("\t", unescape(r"\t"));
}


#[test]
fn vertical_tab_escape_should_produce_vertical_tab() {
    assert_eq!("\u{0b}", unescape(r"\v"));
}
