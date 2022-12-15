use std::{env, fs};

fn main() {
    // this reads arguments and stores them in a vector called args
    println!("-------------------------------------------------");
    let args: Vec<String> = env::args().collect();

    
    let query = &args[1];
    let file_path = &args[2];
    println!("searching for {}, in {}", query, file_path);

    let file_content = fs::read_to_string(file_path).expect("problem in reading file");
    println!("with text: \n ------------- \n{} \n -------------", file_content)
}
