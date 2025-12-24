
use std::io::Error;
use regex::Regex;

pub fn read_text_file(file: &str) -> Result<Vec<String>, Error>{
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let mut list: Vec<String> = vec![];
    let file = File::open(file)?;
    let reader = BufReader::new(file);

    // Read lines with iterator
    for line in reader.lines() {
        let line = line?;
        list.push(line);
    }
    Ok(list)
}

pub fn split(pattern: &str,s: String) -> Option<(String, String)> {
    let split_pattern = Regex::new(pattern).unwrap();
    let caps = split_pattern.captures(&s).map(|c| c.extract::<2>());
    match caps {
        Some(c) => Some((c.1[0].to_string(), c.1[1].to_string())),
        _ => None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let result = split(r"(\w)(.+)", "L60".to_string());
        assert_eq!(result, Some(("L".to_string(), "60".to_string())));
    }
}
