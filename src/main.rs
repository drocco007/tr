use tr::command::tr;


fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let stdin = stdin.lock();
    let mut stdout = stdout.lock();

    let exit_code = match tr(std::env::args(), stdin, &mut stdout) {
        Err(message) => {
            eprintln!("tr: {}", message);
            1
        },
        _ => 0
    };

    std::process::exit(exit_code);
}
