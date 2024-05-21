use std::env;
use std::process;

// minigrep is the lib.rs declared library/crate
// Config is a struct from there!
use minigrep::Config;

fn main() {
    // "arrow functions"
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);     // remember in C returning 0
    });

    // if Err(e) can recieve a casted '()' version of the returned value
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
