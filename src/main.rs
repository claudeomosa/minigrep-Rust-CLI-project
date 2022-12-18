use std::{env, process};
use minigrep::Config;


fn main() {
    println!("-------------------------------------------------");
    // this reads arguments and stores them in a vector called args
    // let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });


    if let Err(e) = minigrep::run(config) {
        eprintln!("app error: {e}");
        process::exit(1)
    }
    println!("-------------------------------------------------");
}
