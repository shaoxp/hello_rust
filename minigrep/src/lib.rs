use std::fs;
use std::error::Error;
use std::env;

pub struct Config{
    pub query: String,
    pub file_name: String,
    pub case_sensitive: bool,
}

impl Config{
    pub fn new(mut args: env::Args) -> Result<Config,&'static str> {
        args.next();

        let query = match args.next(){
            Some(k) => k,
            None => return Err("did not get query"),
        };

        let file_name = match args.next(){
            Some(k) => k,
            None => return Err("did not get file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{
            query,file_name,case_sensitive
        })
    }
}


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    
    let contents = fs::read_to_string(config.file_name)?;

    let results = if config.case_sensitive{
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}",line);
    }

    Ok(())
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line|line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str, 
    contents:&'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents.lines()
        .filter(|line|{
            let lline = line.to_lowercase();
            lline.contains(&query)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_case_sensistive() {
        let query = "duct";
        let contents = "a \
        Rust:
safe,fast,productive
Pick three.
Duct tape";

        assert_eq!(vec!["safe,fast,productive"],search(query,contents));
    }

    #[test]
    fn search_case_insensistive() {
        let query = "rUSt";
        let contents = "\
        Rust:
safe,fast,productive
Pick three.
Trust me";

        assert_eq!(vec!["Rust:","Trust me"],search_case_insensitive(query,contents));
    
    }
}