use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;
use minigrep::search_case_insensitive;
use std::path::Path;

fn main() {
     let args: Vec<String> = env::args().collect();
     let config = Config::new(&args);

     if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1)
     }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
     search_directory(Path::new("."), &config)?;
     Ok(())
     }
     
fn search_directory(path:&Path, config: &Config) -> Result<(), Box<dyn Error>> {
    let entries = fs::read_dir(path)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            if path.file_name().and_then(|n| n.to_str()) == Some(".git") {
            continue;
        }
            search_directory(&path, config)?;


        }else if path.is_file() 
        && path.extension().and_then(|ext| ext.to_str()) == Some("txt")  {
            if let Ok(contents) = fs::read_to_string(&path) {

            
            let results = if config.ignore_case {
                search_case_insensitive(&config.query, &contents)
            }else{
                search(&config.query, &contents)
            };

            for line in results {
                println!("{}: {} ", path.display(), line);
            }
           }
        }
        }
        Ok(())
    }
    
     




struct Config {
    query: String,
    ignore_case: bool,
}

impl Config{
     fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            panic!("Not enough arguments, arguments must be up to 2.");
        }
      let query = args[1].clone();
      let ignore_case = env::var("IGNORE_CASE").is_ok();

    Config { 
        query,
        ignore_case,
     }
    }
}