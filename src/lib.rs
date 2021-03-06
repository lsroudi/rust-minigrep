//! # This project is for training and demonstration
//!
//! `minigrep` is a collection of function similar to grep cmd 
//! given an expression and a file name, minigrep extract lines with the "expr"


use std::error::Error;
use std::fs;
use std::env;

pub struct Config{
    pub query : String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Self,&'static str> { 

        if args.len() < 3 {
            return Err("not enough parameters");
        }

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Ok(Self { query : args[1].clone(), 
            filename : args[2].clone(), 
            case_sensitive: case_sensitive
        })
    }
}

pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
/// Search one expr in the line
/// 
/// Example 
/// ```
/// let query = "rust";
/// let contents = "\
//Rust:
///safe, fast, productive.
///Pick three.
///Trust me.";
/// 
/// assert_eq!(
/// vec!["safe, fast, productive."],
/// search(query, contents)
/// )
pub fn search<'a>(query : &str,contents: &'a str) -> Vec<&'a str>{
    
    contents.lines()
    .filter(|line| line.contains(&query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    println!("query is : {}",query);
    contents.lines()
    .filter(|line| line.to_lowercase().contains(&query))
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

       assert_eq!(
           vec!["safe, fast, productive."],
           search(query, contents)
       );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";


       assert_eq!(
           vec!["Rust:", "Trust me."],
           search_case_insensitive(query, contents)
       );
    }
}


