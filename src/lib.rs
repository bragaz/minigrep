use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    // constructor
    pub fn new_config(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        // we started from one because the vector take as index 0 value the program's name
        return Ok(Config {
            query: args[1].clone(),
            filename: args[2].clone(),
        });
    }
}

// run tries to manage the config of the program and returns a generic error if something
// goes wrong, otherwise it will return the unit type
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

// search look up for query inside the given contents and returns a vector of
// any repetition of query itself
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut res = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            res.push(line);
        }
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "mani";
        let contents = "ini, mini, mani, mo";
        assert_eq!(vec!["ini, mini, mani, mo"], search(query, contents));
    }
}

