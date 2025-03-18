mod test;

use std::env;
use std::env::Args;
use std::f32::consts::E;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;




pub fn search_file(config: &Config)-> Vec<String>{
    let mut line_lowercase;
    let mut result:Vec<String> = Vec::new();
    let mut search_string:String;
    if config.case_sensitive == true{
        search_string = config.search_string.clone().to_lowercase();
    }
    else {
        search_string = config.search_string.clone();

    }


    println!("Trying to open file: {}", config.search_path);
    let file = File::open(&config.search_path).unwrap_or_else(|_| {
        println!("File to Search does not exist");
        exit(1);
    });

            let reader = BufReader::new(file);
            for line in reader.lines(){
                let mut line = line.unwrap_or_else(|_| {
                    println!("Could not Read Line in File");
                    exit(1);
                });

                if config.case_sensitive == true{
                    line_lowercase = line.to_lowercase();

                }

               if line_lowercase.contains(&search_string){
                   result.push(line)

        }

    }
    result
}



pub struct Config{
    pub search_string: String,
    pub search_path: String,
    pub case_sensitive: bool

}
impl Config {
    pub fn new(args: &Vec<String>) -> Self {
        let mut is_case_sensitiv:bool = false;

        if args.len() != 3 {
            if args.len() != 4 && args[3] != "IGNORE_CASE" {
                println!("Please provide all necessary CLI arguments: cargo run [search_string] [search_path] Hello");
                exit(1);

            }
            else {
                is_case_sensitiv = true;
            }


        }


        let search_string = args[1].clone();
        let search_path = args[2].clone();

        // Return a Config instance
        Config {
            search_string,
            search_path,
            case_sensitive: is_case_sensitiv,
        }
    }
}


fn main() {}