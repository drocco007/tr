use std::borrow::Cow;
use std::collections::HashMap;


enum ParseState {
    SkipProgname,
    HelpRequested,
    VersionRequested,
    ParseOptionsAndSet1,
    NextArgIsSet1,
    Set1Written,
    Set2Written,
    ExtraArgs
}


#[derive(Debug,Default)]
pub struct Config {
    pub complement: bool,
    pub delete: bool,
    pub squeeze: bool,
    pub truncate: bool,
    pub help_requested: bool,
    pub version_requested: bool,
    pub set1: String,
    pub set2: String,
    pub first_extra_arg: String,
}


/// Create a mapping from each char in set1 to the corresponding char
/// in set2.
///
/// # Examples
///
/// ```
/// let map = tr::parser::map_charsets("abcde", "zyxwv");
///
/// assert_eq!(&'x', map.get(&'c').unwrap());
/// ```
pub fn map_charsets(set1: &str, set2: &str) -> HashMap<char, char> {
    let (set1, set2) = (unescape(set1), unescape(set2));

    let set2 = rpad_last(&set2, set1.len());

    set1.chars().zip(set2.chars()).collect()
}


/// Extend s to length n by repeating the last char.
///
/// # Examples
///
/// ```
/// assert_eq!("Rust!!!", tr::parser::rpad_last("Rust!", 7));
/// ```
///
/// Returns s unmodified if n is <= s.len():
///
/// ```
/// assert_eq!("why?", tr::parser::rpad_last("why?", 0));
/// assert_eq!("too small", tr::parser::rpad_last("too small", 4));
/// ```
pub fn rpad_last<'a>(s: &'a str, n: usize) -> Cow<'a, str> {
    if s.len() < n {
        let mut buf = String::with_capacity(n);
        let n = n - s.len();
        let c = s.chars().rev().nth(0).expect("empty source string");

        buf.push_str(s);
        buf.push_str(&c.to_string().repeat(n));

        buf.into()
    } else {
        s.into()
    }
}


/// Replace escape sequences in s with the corresponding char.
///
///    \\     backslash
///    \a     audible BEL
///    \b     backspace
///    \f     form feed
///    \n     new line
///    \r     return
///    \t     horizontal tab
///    \v     vertical tab
///
/// A backslash followed by any other char is replaced with that char; the
/// backslash is consumed and not reflected in the output.
///
/// # Examples
///
/// ```
/// assert_eq!("\n", tr::parser::unescape("\\n"));
/// assert_eq!("\n", tr::parser::unescape(r"\n"));
/// assert_eq!("x", tr::parser::unescape(r"\x"));
/// ```
pub fn unescape<'a>(s: &'a str) -> Cow<'a, str> {
    let (mut first, mut rest);

    if let Some(index) = s.find(r"\") {
        first = &s[..index];

        // index+1 -> skip the backslash
        rest = &s[index+1..];
    } else {
        return s.into();
    }

    let mut output = String::with_capacity(s.len());

    loop {
        output.push_str(first);

        let c = &rest[..1];

        // consume the char from the input
        rest = &rest[1..];

        match c {
            "a" => output.push_str("\u{07}"),
            "b" => output.push_str("\u{08}"),
            "f" => output.push_str("\u{0c}"),
            "n" => output.push_str("\n"),
            "r" => output.push_str("\r"),
            "t" => output.push_str("\t"),
            "v" => output.push_str("\u{0b}"),
            _ => output.push_str(c)
        }

        if let Some(index) = rest.find(r"\") {
            first = &rest[..index];

            // skip the backslash
            rest = &rest[index+1..];
        } else {
            output.push_str(rest);
            break;
        }
    }

    output.into()
}


/// Parse an option from the command line.
///
/// Interpret an arg (e.g. passed from the command line) and set the
/// corresponding named flag in the config.
///
/// tr expects all its options to preceed the translation sets, and this
/// function assumes it is called in the same order as the arguments.
///
/// To avoid ambiguity, set1 may not start with '-' _unless_ '--' has
/// been used on the command line to indicate end of options:
///
///    tr -- '-asdf' '*'
///
/// However, the single character '-' is valid as set1:
///
///    tr '-' '*'
///
fn parse_option<'a>(config: &mut Config, arg: &str) -> Result<ParseState, String> {
    use ParseState::*;

    let mut result = Ok(ParseOptionsAndSet1);

    let is_option = arg.len() >= 2 && &arg[..2] == "--";
    let is_switch = !is_option && arg.len() > 1 && &arg[..1] == "-";

    if is_option {
        match arg {
            "--" => result = Ok(NextArgIsSet1),
            "--help" => {
                config.help_requested = true;
                result = Ok(HelpRequested);
            },
            "--version" => {
                config.version_requested = true;
                result = Ok(VersionRequested);
            },
            "--complement" => config.complement = true,
            "--delete" => config.delete = true,
            "--squeeze-repeats" => config.squeeze = true,
            "--truncate-set1" => config.truncate = true,
            _ => result = Err(format!("unrecognized option '{}'", arg))
        }
    } else if is_switch {
        for c in arg[1..].chars() {
            match c {
                'c' | 'C' => config.complement = true,
                'd' => config.delete = true,
                's' => config.squeeze = true,
                't' => config.truncate = true,
                _ => {
                    result = Err(format!("invalid option -- '{}'", c));
                    break;
                }
            }
        }
    } else {
        // options exhausted; current arg is set1
        config.set1 = arg.to_owned();
        result = Ok(Set1Written);
    }

    result
}


/// Parse program arguments
///
/// Returns a Config struct initialized according to the supplied list of
/// arguments if each argument is understood by tr and the combination of
/// arguments is coherent.
///
/// Returns Err("message") on encountering an unrecognized option or if
/// the combined arguments do not make sense.
pub fn parse_args<'a, I>(args: I) -> Result<Config, String>
where
    I: IntoIterator,
    I::Item: AsRef<str>
{
    use ParseState::*;

    let mut state = SkipProgname;
    let mut config: Config = Default::default();

    for arg in args {
        let arg = arg.as_ref();

        match state {
            SkipProgname => {
                state = ParseOptionsAndSet1;
                continue;
            },
            ParseOptionsAndSet1 => {
                match parse_option(&mut config, &arg) {
                    Ok(newstate) => state = newstate,
                    Err(e) => { return Err(e); }
                }
            },
            HelpRequested | VersionRequested => {
                break;
            },
            NextArgIsSet1 => {
                config.set1 = arg.to_owned();
                state = Set1Written;
            },
            Set1Written => {
                config.set2 = arg.to_owned();
                state = Set2Written;
            },
            Set2Written => {
                config.first_extra_arg = arg.to_owned();
                state = ExtraArgs;
                break;
            },
            ExtraArgs => unreachable!()
        }
    }

    // validate coherence of final configuration
    match state {
        ExtraArgs => {
            Err(format!("extra operand ‘{}’", config.first_extra_arg))
        },
        ParseOptionsAndSet1 => {
            Err("missing operand".to_owned())
        },
        Set1Written => {
            // squeeze OR delete Ok, squeeze AND delete requires set2
            match config.squeeze ^ config.delete {
                true => Ok(config),
                false => Err(format!("missing operand after ‘{}’", config.set1))
            }
        },
        _ => Ok(config)
    }
}
