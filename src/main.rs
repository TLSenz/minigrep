use std::env;
use std::env::{args, Args};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;
mod lib;
use crate::lib::{get_args, search_file, Config};

fn main() {
    let args:Vec<String> = env::args().collect();
    let config: Config = Config::new(&args);
    println!("{}", config.search_string);
    println!("{}", config.search_path)




}

