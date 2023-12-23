use std::env;
use std::process;
use minigrep::Config;
use minigrep::run;


fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.get_query());
    println!("In file {}", config.get_file_path());

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

