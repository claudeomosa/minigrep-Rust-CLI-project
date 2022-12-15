use std::{env, process};
use minigrep::Config;


fn main() {
    println!("-------------------------------------------------");
    // this reads arguments and stores them in a vector called args
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing the arguments: {}", err);
        process::exit(1)
    });
    println!("searching for {}, in {}", config.query, config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("ERROR: {e}");
        process::exit(1)
    }
}
