use std::{env, process};
use minigrep;

fn main() {
    // unwrap_or_else read the Result<T,E> and return the type if there's some or the error
    let config = minigrep::Config::new_config(env::args())
        .unwrap_or_else(|err| {
        eprintln!("{}", err);
        // this line stops the execution immediately
        process::exit(1);
    });

    // here we don't use unwrap_or_else cause we don't care about anything but the error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

