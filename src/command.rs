use unicode_reader::CodePoints;

use crate::arg_parser::Config;
use crate::parser::map_charsets;


pub fn show_help() {
    println!("Usage: tr [OPTION]... SET1 [SET2]");
}


pub fn show_version() {
    println!("tr[ust] 0.9");
}


pub fn translate(config: &Config) {
    let map = map_charsets(&config.set1, &config.set2);

    let stdin = std::io::stdin();

    let input = CodePoints::from(stdin.lock());

    for c in input {
        let c = c.unwrap();

        let c = map.get(&c).unwrap_or(&c);

        print!("{}", c);
    }
}
