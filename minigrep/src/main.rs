use std::env;
use std::process;

use::minigrep::Config;

fn main() {
    // std::env::args returns an iterator of the command line args
    // collect() method turns an iterator inro collection, such as Vec
    // we explicitly annotate that we need vector of strings because Rust isn't able to infer
    // what kind of collection we need
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // we don’t need unwrap_or_else to return the unwrapped value, 
    // from run because it can only be ()
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
