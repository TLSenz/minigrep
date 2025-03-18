#[cfg(test)]


mod tests {
    use crate::search_file;

    #[test]
    fn test_search() {
        let  query = "duct";
        let contents = "\
        Rust:\
        safe, fast, productive.\
        Pick three.";
        assert!(vec!["safe, fast, productive."],search_file(query,contents));
    }
}