use std::{env, process, fs};

use config::Config;
use source::SourceAdapter;
mod config;
mod source;

fn main() {
    let args = env::args();
    let config = Config::parse(args).unwrap_or_else(|e| {
        eprintln!("Error while retrieving config: {}", e);
        process::exit(1);
    });

    let source = SourceAdapter::new(fs::read_to_string).from_config(config);

    println!("Got that: {:?}", source.unwrap())
}
