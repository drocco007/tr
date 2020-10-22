use rstest::rstest;

use tr::lex::tokenize;


#[test]
fn tokenizer_should_be_iterator() {
    let mut tokens = tokenize("");

    assert_eq!(tokens.next(), None);
}


#[rstest(
    s => ["a", "z", "1", "\\"]
)]
fn tokenizer_should_return_single_literal(s: &str) {
    let mut tokens = tokenize(s);

    assert_eq!(tokens.next().unwrap(), s);
}


#[rstest(
    s => ["a", "z", "1", "\\"]
)]
fn single_literal_should_end_token_stream(s: &str) {
    let mut tokens = tokenize(s);

    tokens.next();
    assert_eq!(tokens.next(), None);
}


#[rstest(
    s => ["qwert", "yuiop", "0xdeadbeef", "#334455"]
)]
fn tokenizer_should_return_string_literal(s: &str) {
    let mut tokens = tokenize(s);

    assert_eq!(tokens.next().unwrap(), s);
}


#[rstest(
    s => ["qwert", "yuiop", "0xdeadbeef", "#334455"]
)]
fn string_literal_should_end_token_stream(s: &str) {
    let mut tokens = tokenize(s);

    tokens.next();
    assert_eq!(tokens.next(), None);
}


// FIXME: tr actual treats pattern "2[.*a]3" as an error
#[rstest(
    s => ["a[::]z", "0[:abcd", "-", "-q", "a[:xdigi:]z", "0[xx*]9",
          "Z[:alnum]A", "A-"]
)]
fn pseudo_repeats_and_classes_should_be_treated_as_literals(s: &str) {
    let mut tokens = tokenize(s);

    assert_eq!(tokens.next().unwrap(), s);
}


#[rstest(
    s => [r"\\", r"\a", r"\b", r"\f", r"\n", r"\r", r"\t", r"\v"]
)]
fn tokenizer_should_return_initial_backslash_escape(s: &str) {
    let mut tokens = tokenize(s);

    assert_eq!(tokens.next().unwrap(), s);
}


#[rstest(
    s => [r"\\", r"\a", r"\b", r"\f", r"\n", r"\r", r"\t", r"\v"]
)]
fn initial_backslash_escape_should_end_token_stream(s: &str) {
    let mut tokens = tokenize(s);

    tokens.next();
    assert_eq!(tokens.next(), None);
}


#[test]
fn solo_range_should_tokenize_togther() {
    let mut tokens = tokenize("a-z");

    assert_eq!(tokens.next().unwrap(), "a-z");
}


#[test]
fn inital_range_should_tokenize_togther() {
    let mut tokens = tokenize("a-z0-");

    assert_eq!(tokens.next().unwrap(), "a-z");
}


#[test]
fn final_range_should_tokenize_togther() {
    let mut tokens = tokenize("asdfqwert0-9");

    assert_eq!(tokens.next().unwrap(), "asdfqwert");
    assert_eq!(tokens.next().unwrap(), "0-9");
}


#[test]
fn interior_range_should_tokenize_togther() {
    let mut tokens = tokenize("asdfqwert0-9uiop");

    assert_eq!(tokens.next().unwrap(), "asdfqwert");
    assert_eq!(tokens.next().unwrap(), "0-9");
    assert_eq!(tokens.next().unwrap(), "uiop");
}


#[test]
fn dangling_open_bracket_followed_by_legitimate_range() {
    let tokens = tokenize("[fq-z").collect::<Vec<&str>>();

    assert_eq!(tokens, vec!["[f", "q-z"]);
}


#[test]
fn complicated_scenario() {
    let s = "\\0ab[=c=]def[.*]as[d*20][**][:*30][fq-z0-9]\\tX-";
    let result = tokenize(s).collect::<Vec<&str>>();
    let expected = vec!["\\0", "ab", "[=c=]", "def", "[.*]", "as", "[d*20]",
                        "[**]", "[:*30]", "[f", "q-z", "0-9", "]", "\\t",
                        "X-"];

    assert_eq!(result, expected);
}


#[test]
fn complicated_octal_parsing_scenario() {
    let s = "\\0asdf[:xdigi:]jkl\\01\\012\\0123\\9\\09\\019[::]X";

    let result = tokenize(s).collect::<Vec<&str>>();
    let expected = vec!["\\0", "asdf[:xdigi:]jkl", "\\01", "\\012", "\\012",
                        "3", "\\9", "\\0", "9", "\\01", "9[::]X"];

    assert_eq!(result, expected);
}
