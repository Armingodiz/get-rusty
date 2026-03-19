use std::env::args;
use std::fs;
use std::process;
use std::error::Error;
use minigrep::search;

fn main() {
    let args: Vec<String> = args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    run(config).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });
}


fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    let contents = fs::read_to_string(config.filename)?;
    println!("With text:\n{}", contents);
    println!("-------------------------------- Result --------------------------------");
    let results = search(&config.query, &contents);
    for line in results {
        println!("{line}");
    }
    Ok(())
}

struct Config {
    query: String,
    filename: String,
}


impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}