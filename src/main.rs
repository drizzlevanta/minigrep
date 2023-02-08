use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    dbg!(&args);
    //save values in variables
    // let query = &args[1];
    // let file_path = &args[2];

    // println!("Searching for {}", query);
    // println!("In file {}", file_path);
    // let config = parse_config(&args);
    // let config = Config::new(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
    //TODO the following line has borrow of partially moved value
    //println!("config:{:?}", config);
}
