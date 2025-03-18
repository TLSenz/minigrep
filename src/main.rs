use std::env;
use std::env::args;
use std::process::exit;
use minigrep::{search_file, Config};

fn main() {
    let args:Vec<String> = args().collect();

    let config:Config = Config::new(&args);
    let result:Vec<String> = search_file(&config);

    for n in result.iter(){
        println!("{}",n);
    }

}
fn get_args() -> Vec<String>{
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

fn display_args(search_string: &String, search_path: &String){
    println!("Search Query: {}", search_string);
    println!("Search Path: {}", search_path)
}

