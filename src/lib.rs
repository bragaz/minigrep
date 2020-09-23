use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}

impl Config {
    // constructor
    pub fn new_config(mut args: env::Args) -> Result<Config, &'static str> {
        // skipping the first argument which is the program's name
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // we started from one because the vector take as index 0 value the program's name
        return Ok(Config {
            query,
            filename,
            case_sensitive,
        });
    }
}

// run tries to manage the config of the program and returns a generic error if something
// goes wrong, otherwise it will return the unit type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

// search look up for query inside the given contents and returns a vector of
// any repetition of query itself
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
   contents.
       lines()
       .filter(|line| line.contains(&query) )
       .collect()
}

// search_case_insensitive look up for query inside the given content and returns a vector of
// any repetition of query. This one is case insensitive.
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "mani";
        let contents = "\
        ini, \
        mini, \
        mani, \
        mo";
        assert_eq!(vec!["ini, mini, mani, mo"], search(query, contents));
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

