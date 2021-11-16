use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("query: {}", config.query);
    println!("filename: {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("error occur: {}", e);
        process::exit(1);
    }
}
