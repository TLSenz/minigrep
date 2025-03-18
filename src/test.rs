#[cfg(test)]


mod tests {
    use crate::{search_file, Config};

    #[test]
    fn search_case_sensitive() {
        let  query = "duct";
        let contents = "C:/users/svenz/RustroverProjects/minigrep/src/hallo.txt";
        let args:Vec<String> = vec!["Hello".parse().unwrap(), query.clone().parse().unwrap(), contents.clone().parse().unwrap()];

        let config:Config = Config::new(&args);
        assert_eq!(vec!["safe, fast, productive."], search_file(&config));
    }


    #[test]
    fn serch_case_insensitve() {
        let  query = "rUsT";
        let contents = "C:/users/svenz/RustroverProjects/minigrep/src/hallo.txt";
        let args:Vec<String> = vec!["Hello".to_string(), query.clone().parse().unwrap(), contents.clone().parse().unwrap(),"IGNORE_CASE".to_string()];

        let config:Config = Config::new(&args);
        assert_eq!(vec!["Rust:"], search_file(&config));

    }
}