use std::env;
use std::process;
use rgrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e) = rgrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
