use std::io;
use std::iter::Iterator;

use tr::map_charsets;
use unicode_reader::CodePoints;


fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let map = map_charsets(&args[1], &args[2]);

    let stdin = io::stdin();

    let input = CodePoints::from(stdin.lock());

    for c in input {
        let c = c?;

        let c = map.get(&c).unwrap_or(&c);

        print!("{}", c);
    }

    Ok(())
}
