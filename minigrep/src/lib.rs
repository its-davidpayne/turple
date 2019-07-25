use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

 
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        if args.len() > 3 {
            return Err("Too many arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<Error>> {
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
    
    #[test]
    fn blankety_blank() {
        let query = "";
        let contents = "";
        let emptyvec: Vec<String> = Vec::new();
        
        assert_eq!(emptyvec, search(query, contents));
    }

    #[test]
    fn missing_clargs() {
        let args = vec!["only_argument".to_string()];
        let config = Config::new(&args);
        
        assert!(config.is_err());
    }

    #[test]
    fn extra_clargs() {
        let args = vec!["one".to_string(),
                        "two".to_string(), 
                        "three".to_string(), 
                        "four".to_string()];
        let config = Config::new(&args);

        assert!(config.is_err());
    }
}  