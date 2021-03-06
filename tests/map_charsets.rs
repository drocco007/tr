use tr::parser::map_charsets;

use rstest::rstest;


#[test]
fn mapping_empty_charsets_should_succeed() {
    map_charsets("", "");
}


#[test]
fn should_map_unit_length_set1_to_set2() {
    let map = map_charsets("a", "z");

    assert_eq!(&"z", map.get("a").unwrap());
}


#[test]
fn should_map_same_length_set1_to_set2() {
    let map = map_charsets("abcde", "zyxwv");

    assert_eq!(&"z", map.get("a").unwrap());
    assert_eq!(&"y", map.get("b").unwrap());
    assert_eq!(&"x", map.get("c").unwrap());
    assert_eq!(&"w", map.get("d").unwrap());
    assert_eq!(&"v", map.get("e").unwrap());
}


#[test]
fn extraneous_chars_in_set2_should_be_ignored() {
    let map = map_charsets("a", "zEXTRA!EXTRA!");

    assert_eq!(&"z", map.get("a").unwrap());
}


#[test]
fn should_map_unicode_to_unicode() {
    let map = map_charsets("é", "É");

    assert_eq!(&"É", map.get("é").unwrap());
}


#[test]
fn should_map_unicode_to_ascii() {
    let map = map_charsets("é", "#");

    assert_eq!(&"#", map.get("é").unwrap());
}


#[test]
fn should_map_ascii_to_unicode() {
    let map = map_charsets("*", "É");

    assert_eq!(&"É", map.get("*").unwrap());
}


#[rstest(
    case => [("é", "É"), ("a", "j"), ("s", "k"), ("d", "l"), ("f", ";"),
             ("♥", "%"), ("!", "¡"), ("1", "0")]
)]
fn unicode_in_mapping_should_preserve_character_length(case: (&str, &str)) {
    let (source, target) = case;
    let map = map_charsets("éasdf♥!1", "Éjkl;%¡0");

    assert_eq!(&target, map.get(source).unwrap());
}


#[test]
#[ignore]
fn should_extend_set2_to_set1_length() {
    let map = map_charsets("1234567890", ".");

    assert_eq!(&".", map.get("2").unwrap());
    assert_eq!(&".", map.get("0").unwrap());
}


#[test]
fn should_map_escape_in_set1() {
    let map = map_charsets(r"\a", "@");

    assert_eq!(&"@", map.get("\u{07}").unwrap());
}


#[test]
fn should_map_escapes_in_set1() {
    let map = map_charsets(r"\n\t\v\b\r", "01234");

    assert_eq!(&"0", map.get("\n").unwrap());
    assert_eq!(&"1", map.get("\t").unwrap());
    assert_eq!(&"2", map.get("\u{0b}").unwrap());
    assert_eq!(&"3", map.get("\u{08}").unwrap());
    assert_eq!(&"4", map.get("\r").unwrap());
}


#[test]
fn should_map_escapes_in_mixed_set1() {
    let map = map_charsets(r" \n\t+/|", "· t_=-");

    println!("");
    println!("");
    println!("{:?}", map);
    println!("");
    println!("");
    assert_eq!(&"·", map.get(" ").unwrap());
    assert_eq!(&" ", map.get("\n").unwrap());
    assert_eq!(&"t", map.get("\t").unwrap());
    assert_eq!(&"_", map.get("+").unwrap());
    assert_eq!(&"=", map.get("/").unwrap());
    assert_eq!(&"-", map.get("|").unwrap());
}


#[test]
fn should_map_escape_in_set2() {
    let map = map_charsets(r"\a", "@");

    assert_eq!(&"@", map.get("\u{07}").unwrap());
}


#[test]
fn should_map_escapes_in_set2() {
    let map = map_charsets("qwert", r"\n\t\v\b\r");

    assert_eq!(&"\n", map.get("q").unwrap());
    assert_eq!(&"\t", map.get("w").unwrap());
    assert_eq!(&"\u{0b}", map.get("e").unwrap());
    assert_eq!(&"\u{08}", map.get("r").unwrap());
    assert_eq!(&"\r", map.get("t").unwrap());
}


#[test]
fn should_map_escapes_in_mixed_set2() {
    let map = map_charsets("·^v_=-", r" \n\t+|/");

    assert_eq!(&" ", map.get("·").unwrap());
    assert_eq!(&"\n", map.get("^").unwrap());
    assert_eq!(&"\t", map.get("v").unwrap());
    assert_eq!(&"+", map.get("_").unwrap());
    assert_eq!(&"/", map.get("-").unwrap());
    assert_eq!(&"|", map.get("=").unwrap());
}
