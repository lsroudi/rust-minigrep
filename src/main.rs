use std::env;
use std::fs;

fn main() {

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    let contents = fs::read_to_string(&config.filename)
        .expect("Something wrong when reading the file");
    
    println!("With text:\n{}", contents);
    println!("the filename is {} ",&config.filename);
}

struct Config{
    query : String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Self { 
        Self { query : args[1].clone(), 
            filename : args[2].clone(), 
        }
    }
}

