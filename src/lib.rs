use std::{ fs, error::Error, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // this method returns an instance of the Config struct from the borrowed args array in the parameters field
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("\nerror: not enough arguments\n");
        }else if args.len() > 4 {
            return Err("\nerror: too many arguments\n")
        }else {
            let query = args[1].clone();
            let file_path = args[2].clone();
            let ignore_case = env::var("IGNORE_CASE").is_ok();
            Ok( Config { query, file_path, ignore_case})
        }
        
    }
}
// this function takes a Config instance and prints file content from the provided fields 
pub fn run(config: Config) -> Result<(), Box<dyn Error >> {
    let file_contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &file_contents)
    } else {
        search_case_sensitive(&config.query, &file_contents)
    };
    
    for line in results{
        println!("{line}")
    }

    Ok(())
}

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line)
        }
    }
    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str>{
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            result.push(line)
        }
    } 
    result
}

// tests module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
Duct tape";
        assert_eq!(vec!["safe, fast, productive"],  search_case_sensitive(query, contents))
    }

    #[test]
    fn case_insensitive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive
Pick three.
trust me";
        assert_eq!(vec!["Rust:", "trust me"], search_case_insensitive(query, contents))
    }
}
