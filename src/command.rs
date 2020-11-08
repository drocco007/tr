use std::collections::HashSet;
use std::io::{BufRead,Write};

use crate::arg_parser::{Config, parse_args};
use crate::parser::{parse,map_charsets};

use bstr::ByteSlice;


pub fn show_help() {
    println!("Usage: tr [OPTION]... SET1 [SET2]");
}


pub fn show_version() {
    println!("tr[ust] 0.9");
}


pub fn process<F>(mut op: F) -> Result<(), std::io::Error>
where
    F: FnMut(&str) -> Option<String>
{
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let mut stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let mut buffer = stdin.fill_buf()?;
    let mut length = buffer.len();

    while !buffer.is_empty() {
        // FIXME: test & handle non-utf-8 input
        // FIXME: handle case where buffer splits a grapheme
        for b in buffer.graphemes() {
            match op(b) {
                Some(c) => { stdout.write(c.as_bytes())?; }
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
    let map = map_charsets(&config.set1, &config.set2);

    process(|b| {
        match map.get(b) {
            Some(c) => Some(c.to_string()),
            _ => Some(b.to_string())
        }
    })
}


pub fn delete(config: &Config) -> Result<(), std::io::Error> {
    let set = parse(&config.set1).as_bytes().graphemes()
        .map(|c| c.to_string())
        .collect::<HashSet<_>>();

    process(|b| match set.contains(b) {
        true => None,
        false => Some(b.to_string())
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
