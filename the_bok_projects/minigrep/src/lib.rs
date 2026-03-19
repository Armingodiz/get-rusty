pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "ducta";
        let contents = "\
Rust:
safe, fast, productive.";
        assert_eq!(vec!["safe, fast, productive."], search(query, &contents));
    }
}