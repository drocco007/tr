use tr::rpad_last;


#[test]
fn should_succeed_with_empty_string() {
    assert_eq!("", rpad_last("", 0));
}


#[test]
fn should_return_input_str_with_of_supplied_length() {
    let s = "fantastic";

    assert_eq!(s, rpad_last(s, s.len()));
}


#[test]
fn should_ignore_length_less_than_input_str_length() {
    let s = "fantastic";

    assert_eq!(s, rpad_last(s, 0));
    assert_eq!(s, rpad_last(s, 3));
    assert_eq!(s, rpad_last(s, s.len() - 1));
}


#[test]
fn should_extend_unit_str_by_one() {
    assert_eq!("..", rpad_last(".", 2));
}


#[test]
fn should_extend_unit_str_by_two() {
    assert_eq!("...", rpad_last(".", 3));
}


#[test]
fn should_extend_str() {
    assert_eq!("asdfffffff", rpad_last("asdf", 10));
}


// TODO: maybe support this
// #[test]
// fn should_extend_unicode_char_str() {
//     assert_eq!("♥♥♥♥♥♥♥♥♥♥", rpad_last("♥", 10));
// }
