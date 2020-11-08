use rstest::rstest;

use tr::lex::tokenize;
use tr::lex::TokenType::{*};


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

    assert_eq!(tokens.next().unwrap().token, s);
}


#[rstest(
    s => ["a", "z", "1", "\\"]
)]
fn single_literal_should_be_of_type_literal(s: &str) {
    let mut tokens = tokenize(s);

    assert_eq!(tokens.next().unwrap().token_type, Literal);
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

    assert_eq!(tokens.next().unwrap().token, s);
}


#[rstest(
    s => ["qwert", "yuiop", "0xdeadbeef", "#334455"]
)]
fn string_literal_should_be_of_type_literal(s: &str) {
    let mut tokens = tokenize(s);

    assert_eq!(tokens.next().unwrap().token_type, Literal);
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
    let token = tokens.next().unwrap();

    assert_eq!(token.token, s);
    assert_eq!(token.token_type, Literal);
}


#[rstest(
    s => [r"\\", r"\a", r"\b", r"\f", r"\n", r"\r", r"\t", r"\v"]
)]
fn tokenizer_should_return_initial_backslash_escape(s: &str) {
    let token = tokenize(s).next().unwrap();
    let expected = tr::lex::unescape(s);

    assert_eq!(token.token, expected);
}


#[rstest(
    s => [r"\\", r"\a", r"\b", r"\f", r"\n", r"\r", r"\t", r"\v"]
)]
fn initial_backslash_escape_should_be_of_type_literal(s: &str) {
    let mut tokens = tokenize(s);

    assert_eq!(tokens.next().unwrap().token_type, Literal);
}


#[rstest(
    s => [r"\\", r"\a", r"\b", r"\f", r"\n", r"\r", r"\t", r"\v"]
)]
fn initial_backslash_escape_should_end_token_stream(s: &str) {
    let mut tokens = tokenize(s);

    tokens.next();
    assert_eq!(tokens.next(), None);
}


#[rstest(
    case => [(r"\0", "\u{0}"), (r"\00", "\u{0}"), (r"\012", "\u{012}"),
             (r"\141", "\u{141}"), (r"\177", "\u{177}")],
    prefix => ["", "asdf", "0-9"],
    suffix => ["", "uiop", "\\n"],
)]
#[ignore]
fn should_tokenize_octal_escape(case: (&str, &str), prefix: &str, suffix: &str) {
    let (s, expected) = case;
    let stream = format!("{}{}{}", prefix, s, suffix);

    for token in tokenize(&stream) {
        if token.token == expected {
            assert_eq!(token.token_type, Literal);
            return;
        }
    }

    panic!("Token '{:?}' not found in stream!", expected);
}


#[test]
fn complicated_octal_parsing_scenario() {
    let s = "\\0asdf[:xdigi:]jkl\\01\\012\\0123\\9\\09\\019[::]X-";

    let result = tokenize(s).map(|t| t.token).collect::<Vec<String>>();
    let expected = vec!["\u{0}", "asdf[:xdigi:]jkl", "\u{1}", "\n", "\n",
                        "3", "9", "\u{0}", "9", "\u{1}", "9[::]X-"];

    assert_eq!(result, expected);
}


#[test]
fn solo_range_should_tokenize_togther() {
    let mut tokens = tokenize("a-z");

    assert_eq!(tokens.next().unwrap().token, "a-z");
}


#[test]
fn solo_range_should_have_type_range() {
    let mut tokens = tokenize("a-z");

    assert_eq!(tokens.next().unwrap().token_type, CharRange);
}


#[test]
fn inital_range_should_tokenize_togther() {
    let mut tokens = tokenize("a-z0-");

    assert_eq!(tokens.next().unwrap().token, "a-z");
}


#[test]
fn final_range_should_tokenize_togther() {
    let mut tokens = tokenize("asdfqwert0-9");

    assert_eq!(tokens.next().unwrap().token, "asdfqwert");
    assert_eq!(tokens.next().unwrap().token, "0-9");
}


#[test]
fn final_range_should_have_type_range() {
    let mut tokens = tokenize("asdfqwert0-9");

    tokens.next();

    assert_eq!(tokens.next().unwrap().token_type, CharRange);
}


#[test]
fn interior_range_should_tokenize_togther() {
    let mut tokens = tokenize("asdfqwert0-9uiop");

    assert_eq!(tokens.next().unwrap().token, "asdfqwert");

    let token = tokens.next().unwrap();

    assert_eq!(token.token, "0-9");
    assert_eq!(token.token_type, CharRange);

    assert_eq!(tokens.next().unwrap().token, "uiop");
}


#[test]
fn dangling_open_bracket_followed_by_legitimate_range() {
    let mut tokens = tokenize("[fq-z");

    let token = tokens.next().unwrap();

    assert_eq!(token.token, "[f");
    assert_eq!(token.token_type, Literal);

    let token = tokens.next().unwrap();

    assert_eq!(token.token, "q-z");
    assert_eq!(token.token_type, CharRange);
}


#[rstest(
    s => ["c", "1", "-", "*"],
    prefix => ["", "asdf", "0-9"],
    suffix => ["", "uiop", "\\n"],
)]
fn should_tokenize_repeat(s: &str, prefix: &str, suffix: &str) {
    let target = format!("[{}*]", s);
    let s = format!("{}{}{}", prefix, target, suffix);

    for token in tokenize(&s) {
        if token.token == target {
            assert_eq!(token.token_type, CharRepeat);
            return;
        }
    }

    panic!("");
}


#[rstest(
    len => ["1", "22", "333", "4444"]
)]
fn should_tokenize_repeat_with_length(len: &str) {
    let target = format!("[q*{}]", len);
    let token = tokenize(&target).next().unwrap();

    assert_eq!(token.token, target);
    assert_eq!(token.token_type, CharRepeat);
}


#[test]
fn tr_actual_treats_repeat_with_cardinality_zero_as_repeat() {
    let token = tokenize("[.*0]").next().unwrap();

    assert_eq!(token.token_type, CharRepeat);
}


#[rstest(
    s => ["c", "1", "-", "*"],
    prefix => ["", "asdf", "0-9"],
    suffix => ["", "uiop", "\\n"],
)]
fn should_tokenize_equivalence(s: &str, prefix: &str, suffix: &str) {
    let target = format!("[={}=]", s);
    let s = format!("{}{}{}", prefix, target, suffix);

    for token in tokenize(&s) {
        if token.token == target {
            assert_eq!(token.token_type, Equivalence);
            return;
        }
    }

    panic!("");
}


#[rstest(
    s => ["[:alnum:]", "[:alpha:]", "[:blank:]", "[:cntrl:]", "[:digit:]",
          "[:graph:]", "[:lower:]", "[:print:]", "[:punct:]", "[:space:]",
          "[:upper:]", "[:xdigit:]"],
    prefix => ["", "asdf", "0-9"],
    suffix => ["", "uiop", "\\n"],
)]
fn should_tokenize_class(s: &str, prefix: &str, suffix: &str) {
    let stream = format!("{}{}{}", prefix, s, suffix);

    for token in tokenize(&stream) {
        if token.token == s {
            assert_eq!(token.token_type, CharClass);
            return;
        }
    }

    panic!("");
}


#[test]
#[ignore]  // oh brother…
fn escape_sequence_should_be_valid_equivalence_char() {
    let token = tokenize("[=\\n=]").next().unwrap();

    assert_eq!(token.token_type, Equivalence);
}


#[test]
#[ignore]  // oh brother…
fn octal_escape_sequence_should_be_valid_equivalence_char() {
    let token = tokenize("[=\\012=]").next().unwrap();

    assert_eq!(token.token_type, Equivalence);
}


#[test]
fn should_treat_empty_equivalence_as_literal() {
    let token = tokenize("[==]").next().unwrap();

    assert_eq!(token.token, "[==]");
    assert_eq!(token.token_type, Literal);
}


#[test]
fn complicated_scenario() {
    let s = "\\0ab[=c=]def[.*]as[d*20][**][:*30][fq-z0-9]\\tX-";

    let (tokens, token_types): (Vec<_>, Vec<_>) =
        tokenize(s)
        .map(|t| (t.token, t.token_type))
        .unzip();

    let expected = vec![
        "\u{0}", "ab", "[=c=]", "def", "[.*]", "as",
        "[d*20]", "[**]", "[:*30]", "[f", "q-z", "0-9",
        "]", "\t", "X-"
    ];

    let expected_types = vec![
        Literal, Literal, Equivalence, Literal, CharRepeat, Literal,
        CharRepeat, CharRepeat, CharRepeat, Literal, CharRange, CharRange,
        Literal, Literal, Literal
    ];

    assert_eq!(tokens, expected);
    assert_eq!(token_types, expected_types);
}


#[test]
fn unicode_sequence_should_parse() {
    let s = "· t_=-";

    let token = tokenize(s).next().unwrap();

    assert_eq!(token.token_type, Literal);
    assert_eq!(token.token, s);
}
