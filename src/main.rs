use unicode_reader::CodePoints;

use tr::map_charsets;
use tr::command::parse_args;


fn main() {
    let config = match parse_args(std::env::args()) {
        Err(message) => {
            eprintln!("tr: {}", message);
            std::process::exit(1);
        },
        Ok(config) => config
    };

    let map = map_charsets(&config.set1, &config.set2);

    let stdin = std::io::stdin();

    let input = CodePoints::from(stdin.lock());

    for c in input {
        let c = c.unwrap();

        let c = map.get(&c).unwrap_or(&c);

        print!("{}", c);
    }
}
