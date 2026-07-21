use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
     let args: Vec<String> = env::args().collect();
     let config = Config::new(&args);

     if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
     }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
     let contents = fs::read_to_string(config.file_path)?;
     
     let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
     }else{
        search(&config.query, &contents)
     };

     for line in search(&config.query, &contents) {
        println!("{line}");
     }
     Ok(())
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config{
     fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments, arguments must be up to 3.");
        }
      let query = args[1].clone();
      let file_path = args[2].clone();
      let ignore_case = env::var("IGNORE_CASE").is_ok();

    Config { 
        query,
        file_path,
        ignore_case,
     }
    }
}