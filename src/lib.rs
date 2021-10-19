use std::fs;
use std::error::Error;
use std::env;

pub struct Config<'a> {
    pub query: &'a str,
    pub filename: &'a str,
    pub case_sensitive: bool
}

impl<'a> Config<'a> {
    pub fn new(args: &'a Vec<String>) -> Result<Config, &'static str> {

        if args.len() < 3 {
            // Returns the error which is much easier to read than backtracy type stuff
            return Err("Not enough arguments")
        }

        let query = &args[1];
        let filename = &args[2];
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        return Ok(Config{query, filename, case_sensitive})
    }
}

pub fn search<'a>(query: &str, contents: &'a str, case_sensitive:bool) -> Vec<&'a str> {
    
    if case_sensitive {
        contents.lines().filter(|line| line.contains(query)).collect()
    } else {
        let lower_query = query.to_lowercase();
        contents.lines().filter(|line| line.to_lowercase().contains(&lower_query)).collect()
    }
}

pub fn run(config:Config) -> Result<(),Box<dyn Error>> {
    // the ? operator will propogate the error back
    let contents = fs::read_to_string(config.filename)?;
    let results = search(config.query, &contents, config.case_sensitive);

    for result in results {
        println!("{}",result);
    }

    Ok(())
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
Pick three.";
        
        assert_eq!(
            vec!["safe, fast, productive."],
            search(query,contents,true)
        )
    }
    #[test]
    
    fn case_insensitive() {
        let query = "RuSt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
assert_eq!(
            vec!["Rust:"],
            search(query,contents,false)
        )
    }
}