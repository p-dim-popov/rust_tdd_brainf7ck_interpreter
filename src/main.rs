use std::{env, fs, process};

use config::Config;
use source::Source;
mod config;
mod source;

fn main() {
    let args = env::args();
    let config = Config::parse(args).unwrap_or_else(|e| {
        eprintln!("Error while retrieving config: {}", e);
        process::exit(1);
    });

    let source = Source::from_config(config, fs::read_to_string).unwrap_or_else(|e| {
        eprintln!("Couldn't parse source: {e}");
        process::exit(1);
    });

    println!("Got that: {:?}", source);
}
