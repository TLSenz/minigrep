mod test;

use std::env;
use std::env::Args;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

pub fn get_args() -> Vec<String>{
    let mut input:Vec<String> = vec![];
    if env::args().len() != 3 {
        println!("Please Provide all Necessary cli argument: cargo run [search_string][search_path]");
        exit(1)
    }
    for arguments in env::args(){
        input.push(arguments)
    }
    input
}
pub fn display_args(search_string: &String, search_path: &String){
    println!("Search Query: {}", search_string);
    println!("Search Path: {}", search_path)
}

pub fn search_file(search_path: &String)-> Vec<String>{
    println!("Search Path: {}", search_path);
    let file = File::open(search_path).expect("Your File does not exist");
    let reader = BufReader::new(file);
    let mut  text:Vec<String> = Vec::new();
    for line in reader.lines(){
        text.push(line.unwrap());
    }

    text
}

pub struct Config{
    pub search_string: String,
    pub search_path: String,

}
impl Config {
    pub fn new(args: &Vec<String>) -> Self {
        // Check that there are exactly 3 arguments passed
        if args.len() != 3 {
            println!("Please provide all necessary CLI arguments: cargo run [search_string] [search_path]");
            exit(1)
        }

        // Extract the arguments (since we know they exist after the length check)
        let search_string = args[1].clone();  // Clone to take ownership
        let search_path = args[2].clone();

        // Return a Config instance
        Config {
            search_string,
            search_path,
        }
    }
}


fn main() {}