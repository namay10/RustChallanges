use std::env;
use std::error::Error;
use std::fs;
pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_caseinsensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(args) => args,
            None => return Err("Error while reading query"),
        };
        //match and ok_or return are both equivalent in this scenario as ok_or also returns a result type with the error of type &str and config if Ok.
        let filename = args.next().ok_or("Error while reading filename")?;
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
pub fn search_caseinsensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
        Rust:
safe, fast, productive
        Pick three.
        ";
        assert_eq!(vec!["safe, fast, productive"], search(query, contents));
    }
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Trust me.
        ";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_caseinsensitive(query, contents)
        );
    }
}
