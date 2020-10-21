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
