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
    let file_content = fs::read_to_string(config.file_path)?;
    println!("with text: \n ------------- \n{} \n -------------", file_content);
    Ok(())
}