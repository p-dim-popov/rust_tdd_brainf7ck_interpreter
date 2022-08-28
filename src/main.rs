use std::{env, process};

use util::Config;
mod util;

fn main() {
    let args = env::args();
    let config = Config::parse(args).unwrap_or_else(|e| {
        eprintln!("Error while retrieving config: {}", e);
        process::exit(1);
    });

    println!("Got that: {:?}", config)
}
