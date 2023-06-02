use std::{env, fs};

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        return Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        std::process::exit(1);
    });
    dbg!(&config);

    let contents = fs::read_to_string(config.file_path).expect("Could not open the file");
    dbg!(&contents);
}
