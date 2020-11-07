use std::collections::HashSet;
use std::io::{BufRead,Stdin,StdinLock,Write};

use crate::arg_parser::{Config, parse_args};
use crate::parser::{map_charsets, parse};


pub fn show_help() {
    println!("Usage: tr [OPTION]... SET1 [SET2]");
}


pub fn show_version() {
    println!("tr[ust] 0.9");
}


pub fn process<F>(mut op: F) -> Result<(), std::io::Error>
where
    F: FnMut(u8) -> Option<u8>
{
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut buffer = stdin.fill_buf()?;
    let mut length = buffer.len();

    while !buffer.is_empty() {
        for b in buffer {
            match op(*b) {
                Some(c) => { stdout.write(&[c])?; }
                None => continue
            }
        }

        stdin.consume(length);
        buffer = stdin.fill_buf().unwrap();
        length = buffer.len();
    }

    stdout.flush()?;

    Ok(())
}


pub fn translate(config: &Config) -> Result<(), std::io::Error> {
    // FIXME: duplicates map_charsets until we convert everything over
    // to unicode
    let (set1, set2) = (parse(&config.set1), parse(&config.set2));

    let set2 = crate::parser::rpad_last(&set2, set1.len());

    use std::collections::HashMap;
    let map = set1.bytes().zip(set2.bytes()).collect::<HashMap<u8,u8>>();

    process(|b| {
        match map.get(&b) {
            Some(&c) => Some(c),
            _ => Some(b)
        }
    })
}


pub fn delete(config: &Config) -> Result<(), std::io::Error> {
    let set = parse(&config.set1).bytes().collect::<HashSet<u8>>();

    process(|b| match set.contains(&b) {
        true => None,
        false => Some(b)
    })
}


pub fn tr<I>(args: I)
where
    I: IntoIterator,
    I::Item: AsRef<str>
{
    let config = match parse_args(args) {
        Err(message) => {
            eprintln!("tr: {}", message);
            std::process::exit(1);
        },
        Ok(config) => config
    };

    if config.help_requested {
        show_help();
    } else if config.version_requested {
        show_version();
    } else if config.delete {
        delete(&config);
    } else {
        translate(&config);
    }

    std::process::exit(0);
}
