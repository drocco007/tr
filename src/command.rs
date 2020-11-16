use std::collections::HashSet;
use std::io::{BufRead,Write};

use crate::arg_parser::{Config, parse_args};
use crate::parser::{parse,map_charsets};

use bstr::ByteSlice;


struct Tr<R, W, O> {
    reader: R,
    writer: W,
    op: O
}


impl<R, W, O> Tr<R, W, O>
where
    R: BufRead,
    W: Write,
    O: FnMut(&str) -> Option<String>
{
    pub fn process(&mut self) -> Result<(), std::io::Error> {
        let mut buffer = self.reader.fill_buf()?;
        let mut length = buffer.len();

       while !buffer.is_empty() {
            // FIXME: test & handle non-utf-8 input
            // FIXME: handle case where buffer splits a grapheme
            for b in buffer.graphemes() {
                match (self.op)(b) {
                    Some(c) => { self.writer.write(c.as_bytes())?; }
                    None => continue
                }
            }

            self.reader.consume(length);
            buffer = self.reader.fill_buf()?;
            length = buffer.len();
        }

        self.writer.flush()?;

        Ok(())
    }
}


/// Show program help message
pub fn show_help() {
    println!("Usage: tr [OPTION]... SET1 [SET2]");
}



/// Show program version
pub fn show_version() {
    println!("tr[ust] 0.9");
}


/// Translate according to `config`.
///
/// Given a Config, return a function that accepts a Unicode grapheme,
/// translating any grapheme appearing in `config.set1` to the corresponding
/// target grapheme appearing in `config.set2`, otherwise returning the
/// original grapheme.
pub fn translate(config: &Config) -> Box<dyn FnMut(&str) -> Option<String>> {
    let map = map_charsets(&config.set1, &config.set2);

    Box::new(move |b: &str| {
        match map.get(b) {
            Some(c) => Some(c.to_string()),
            _ => Some(b.to_string())
        }
    })
}


/// Delete graphemes according to `config`.
///
/// Given a Config, return a function that accepts a Unicode grapheme,
/// returning None if the grapheme appears in `config.set1`, otherwise
/// returning the original grapheme.
pub fn delete(config: &Config) -> Box<dyn FnMut(&str) -> Option<String>> {
    let set = parse(&config.set1).as_bytes().graphemes()
        .map(|c| c.to_string())
        .collect::<HashSet<_>>();

    Box::new(move |b| match set.contains(b) {
        true => None,
        false => Some(b.to_string())
    })
}


/// `tr` program entry.
///
/// Given an iterator of command line arguments, process the arguments into
/// a Config.
///
/// The argument `--`, if present, ends argument processing and signals that
/// tr should read SET1 and optionally SET2 from the arguments list.
///
/// If `--help` is passed as an option, the program prints a help message and
/// exits.
///
/// If `--version` is passed as an option, the program prints version
/// information and exits.
///
pub fn tr<I, R, W>(args: I, reader: R, writer: &mut W) -> Result<(), String>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
    R: BufRead,
    W: Write
{
    let config = parse_args(args)?;

    if config.help_requested {
        show_help();
    } else if config.version_requested {
        show_version();
    } else {
        let op = if config.delete {
            delete(&config)
        } else {
            translate(&config)
        };

        let mut tr = Tr { reader: reader, writer: writer, op: op };
        tr.process();
    }

    Ok(())
}
