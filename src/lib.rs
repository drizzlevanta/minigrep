use std::{
    env::{self, Args},
    error::Error,
    fs,
};

//tests
#[cfg(test)]
mod tests {
    #[cfg(test)]
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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

//TODO borrow at element level
fn parse_config(args: &[String]) -> Config {
    // let query = &args[1];
    // let file_path = &args[2];
    // Config {
    //     query: query.to_string(), //TODO borrow then convert to ownership
    //     file_path: file_path.to_string(),
    // }
    let query = args[1].clone();
    let file_path = args[2].clone();
    let ignore_case = env::var("IGNORE_CASE").is_ok();
    Config {
        query,
        file_path,
        ignore_case,
    }
}

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Needs two arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Config {
            query,
            file_path,
            ignore_case,
        }
    }

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("Needs two arguments");
        // }
        // let query = args[1].clone();
        // let file_path = args[2].clone();

        args.next();
        let query;
        let file_path;

        match args.next() {
            Some(arg) => query = arg,
            None => return Err("No query string provided"),
        };

        match args.next() {
            Some(arg) => file_path = arg,
            None => return Err("No file path found"),
        }

        // let file_path = args.next().unwrap(); //unwrap will panic

        // if let file_path=args.next()  {
        //     None=>return Err("No file path provided")
        // }

        //check if env is set
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("Needs two arguments");
    //     }
    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     //check if env is set
    //     let ignore_case = env::var("IGNORE_CASE").is_ok();
    //     Ok(Config {
    //         query,
    //         file_path,
    //         ignore_case,
    //     })
    // }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //read file
    let contents = fs::read_to_string(config.file_path)?;
    println!("With text:\n{contents}");
    //search in contents
    // let results = search(&config.query, &contents);

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut v: Vec<&str> = Vec::new();
    // for line in contents.lines() {
    //     //do something with line
    //     if line.contains(query) {
    //         v.push(line);
    //     }
    // }

    //using functional programming style to minimize the amount of mutable state
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
    // for line in contents.lines() {
    //     //do something with line
    //     if line.contains(query) {
    //         v.push(line);
    //     }
    // }
    // v
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    let mut v = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            v.push(line);
        }
    }
    v
}
