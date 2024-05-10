// A minigrep project
/*
make minigrep accept its two command line arguments: the file path and a string to search for.
Example
cargo run -- searchstring example-filename.txt
*/
use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;

fn main() {
    // get commandline arguments
    let args: Vec<String> = env::args().collect();

    let config: minigrep::Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
