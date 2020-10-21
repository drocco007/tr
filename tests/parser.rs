use tr::parser::parse_args;


#[test]
fn simple_two_arguments_should_load_set1_and_set2() {
    let args = parse_args(&["tr", "a", "A"]).unwrap();

    assert_eq!(args.set1, "a");
    assert_eq!(args.set2, "A");
}


#[test]
fn attempting_to_load_three_sets_should_produce_error() {
    let err = parse_args(&["tr", "a", "A", "toolong"]).unwrap_err();

    assert_eq!(err, "extra operand ‘toolong’");
}


#[test]
fn pseudo_switch_as_set1_should_indicate_invalid_option() {
    assert_eq!(parse_args(&["tr", "-a"]).unwrap_err(), "invalid option -- 'a'");
}


#[test]
fn solo_dash_as_set1_should_be_valid() {
    let config = parse_args(&["tr", "-", "*"]).unwrap();

    assert_eq!(config.set1, "-");
}


#[test]
fn pseudo_switch_as_set1_should_indicate_unrecognized_option() {
    assert_eq!(parse_args(&["tr", "--absent"]).unwrap_err(),
               "unrecognized option '--absent'");
}

#[test]
fn missing_set1_should_produce_error() {
    let err = parse_args(&["tr", "-d"]).unwrap_err();

    assert_eq!(err, "missing operand");
}


#[test]
fn multilple_switches_specified_as_one_argument_should_be_allowed() {
    let config = parse_args(&["tr", "-dst", "qwert", "yuiop"]).unwrap();

    assert!(config.delete);
    assert!(config.squeeze);
    assert!(config.truncate);
    assert_eq!(config.set1, "qwert");
}


#[test]
fn multilple_switches_specified_as_two_arguments_should_be_allowed() {
    let config = parse_args(&["tr", "-ds", "-C", "qwert", "yuiop"]).unwrap();

    assert!(config.complement);
    assert!(config.delete);
    assert!(config.squeeze);
    assert_eq!(config.set1, "qwert");
}


#[test]
fn redundant_switch_should_be_allowed() {
    let config = parse_args(&["tr", "-c", "-c", "qwert", "yuiop"]).unwrap();

    assert!(config.complement);
    assert_eq!(config.set1, "qwert");
}


#[test]
fn redundant_switches_and_options_should_be_allowed() {
    let config = parse_args(&["tr", "-c", "-C", "--complement", "qwert", "yuiop"]).unwrap();

    assert!(config.complement);
    assert_eq!(config.set1, "qwert");
}


#[test]
fn unknown_switch_in_combo_should_indicate_correct_invalid_option() {
    assert_eq!(parse_args(&["tr", "-sa"]).unwrap_err(), "invalid option -- 'a'");
}


#[test]
fn double_dash_should_end_option_processing() {
    let config = parse_args(&["tr", "--", "-s", ".S"]).unwrap();

    assert_eq!(config.squeeze, false);
    assert_eq!(config.set1, "-s");
}


#[test]
fn single_set_without_squeeze_or_delete_should_be_invalid() {
    assert_eq!(parse_args(&["tr", "p"]).unwrap_err(), "missing operand after ‘p’");
    assert_eq!(parse_args(&["tr", "--", "-s"]).unwrap_err(), "missing operand after ‘-s’");
    assert_eq!(parse_args(&["tr", "-ct", "nope"]).unwrap_err(), "missing operand after ‘nope’");
}


#[test]
fn single_set_with_delete_should_be_valid() {
    let config = parse_args(&["tr", "-d", "*"]).unwrap();

    assert!(config.delete);
    assert!(config.set2.is_empty());
}


#[test]
fn single_set_with_squeeze_should_be_valid() {
    let config = parse_args(&["tr", "-s", "-"]).unwrap();

    assert!(config.squeeze);
    assert!(config.set2.is_empty());
}


#[test]
fn combined_squeeze_and_delete_without_set2_should_be_invalid() {
    assert_eq!(parse_args(&["tr", "-sd", "oh"]).unwrap_err(), "missing operand after ‘oh’");
}


#[test]
fn combined_squeeze_and_delete_with_set2_should_be_valid() {
    let config = parse_args(&["tr", "-ds", "oh", "kay"]).unwrap();

    assert!(config.squeeze);
    assert!(config.delete);
}
