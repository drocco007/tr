use tr::command::{show_help, show_version, translate};
use tr::parser::parse_args;


fn main() {
    let config = match parse_args(std::env::args()) {
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
    } else {
        translate(&config);
    }

    std::process::exit(0);
}
