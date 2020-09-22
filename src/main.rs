use std::{env, process};
use minigrep;

fn main() {
    // collect convert the CLI iterator to a vector of string
    let args: Vec<String> = env::args().collect();

    // {:?} is the debug formatter and let to print the vector below
    println!("{:?}", args);

    // unwrap_or_else read the Result<T,E> and return the type if there's some or the error
    let config = minigrep::Config::new_config(args.as_slice())
        .unwrap_or_else(|err| {
        println!("{}", err);
        // this line stops the execution immediately
        process::exit(1);
    });

    // here we don't use unwrap_or_else cause we don't care about anything but the error
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

