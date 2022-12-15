use std::{ fs, error::Error};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // this method returns an instance of the Config struct from the borrowed args array in the parameters field
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("\nerror: not enough arguments\n");
        }else if args.len() > 3 {
            return Err("\nerror: too many arguments\n")
        }else {
            let query = args[1].clone();
            let file_path = args[2].clone();
            Ok( Config { query, file_path})
        }
        
    }
}
// this function takes a Config instance and prints file content from the provided fields 
pub fn run(config: Config) -> Result<(), Box<dyn Error >> {
    let file_contents = fs::read_to_string(config.file_path)?;
    
    for line in search(&config.query, &file_contents){
        println!("{line}")
    }

    Ok(())
}

// tests module
mod tests {
    use super::*;

    #[test]
    fn one_result(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive
Pick three.";
        assert_eq!(vec!["safe, fast, productive"],  search(query, contents))
    }
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut vector = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            vector.push(line)
        }
    }
    vector
}