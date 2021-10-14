use std::env;
use std::process;
use rust_minigrep::Config;

fn main() {
    // Collect turns the iterator into a vec
    let args: Vec<String> = env::args().collect();

    let config : Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem Parsing Arguments: {}",err);
        process::exit(1);
    });

    if let Err(e) = rust_minigrep::run(config) {
        println!("{}",e);
        process::exit(1);
    }
}