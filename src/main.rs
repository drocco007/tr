use std::collections::HashMap;
use std::io;
use std::iter::Iterator;

use unicode_reader::CodePoints;


fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let map: HashMap<char, char> = args[1].chars()
        .zip(args[2].chars())
        .collect();

    let stdin = io::stdin();

    let input = CodePoints::from(stdin.lock());

    for c in input {
        let c = c?;

        let c = map.get(&c).unwrap_or(&c);

        print!("{}", c);
    }

    Ok(())
}
